//go:build integration_tests
// +build integration_tests

package integration

import (
	"context"
	"fmt"
	"os"
	"testing"
	"time"

	"github.com/blang/semver/v4"
	"github.com/kong/kubernetes-testing-framework/pkg/clusters"
	"github.com/kong/kubernetes-testing-framework/pkg/clusters/addons/certmanager"
	"github.com/kong/kubernetes-testing-framework/pkg/clusters/addons/loadimage"
	"github.com/kong/kubernetes-testing-framework/pkg/clusters/types/kind"
	"github.com/kong/kubernetes-testing-framework/pkg/environments"
	"k8s.io/apimachinery/pkg/api/errors"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"

	"github.com/bpfd-dev/bpfd/bpfd-operator/internal"
	"github.com/bpfd-dev/bpfd/bpfd-operator/pkg/client/clientset/versioned"
	bpfdHelpers "github.com/bpfd-dev/bpfd/bpfd-operator/pkg/helpers"
)

var (
	ctx        context.Context
	cancel     context.CancelFunc
	env        environments.Environment
	bpfdClient *versioned.Clientset

	// These images should already be built on the node so they can
	// be loaded into kind.
	bpfdImage             = os.Getenv("BPFD_IMG")
	bpfdAgentImage        = os.Getenv("BPFD_AGENT_IMG")
	bpfdOperatorImage     = os.Getenv("BPFD_OPERATOR_IMG")
	tcExampleUsImage      = "quay.io/bpfd-userspace/go-tc-counter:latest"
	xdpExampleUsImage     = "quay.io/bpfd-userspace/go-xdp-counter:latest"
	tpExampleUsImage      = "quay.io/bpfd-userspace/go-tracepoint-counter:latest"
	certmanagerVersionStr = os.Getenv("CERTMANAGER_VERSION")

	existingCluster      = os.Getenv("USE_EXISTING_KIND_CLUSTER")
	keepTestCluster      = func() bool { return os.Getenv("TEST_KEEP_CLUSTER") == "true" || existingCluster != "" }()
	keepKustomizeDeploys = func() bool { return os.Getenv("TEST_KEEP_KUSTOMIZE_DEPLOYS") == "true" }()

	cleanup = []func(context.Context) error{}
)

const (
	bpfdKustomize = "../../config/test"
	bpfdConfigMap = "../../config/bpfd-deployment/config.yaml"
)

func TestMain(m *testing.M) {
	// check that we have the bpfd, bpfd-agent, and bpfd-operator images to use for the tests.
	// generally the runner of the tests should have built these from the latest
	// changes prior to the tests and fed them to the test suite.
	if bpfdImage == "" || bpfdAgentImage == "" || bpfdOperatorImage == "" {
		exitOnErr(fmt.Errorf("BPFD_IMG, BPFD_AGENT_IMG, and BPFD_OPERATOR_IMG must be provided"))
	}

	ctx, cancel = context.WithCancel(context.Background())
	defer cancel()

	// to use the provided bpfd, bpfd-agent, and bpfd-operator images we will need to add
	// them as images to load in the test cluster via an addon.
	loadImages, err := loadimage.NewBuilder().WithImage(bpfdImage)
	exitOnErr(err)
	loadImages, err = loadImages.WithImage(bpfdAgentImage)
	exitOnErr(err)
	loadImages, err = loadImages.WithImage(bpfdOperatorImage)
	exitOnErr(err)
	loadImages, err = loadImages.WithImage(tcExampleUsImage)
	exitOnErr(err)
	loadImages, err = loadImages.WithImage(xdpExampleUsImage)
	exitOnErr(err)
	loadImages, err = loadImages.WithImage(tpExampleUsImage)
	exitOnErr(err)

	if existingCluster != "" {
		fmt.Printf("INFO: existing kind cluster %s was provided\n", existingCluster)

		// if an existing cluster was provided, build a test env out of that instead
		cluster, err := kind.NewFromExisting(existingCluster)
		exitOnErr(err)
		env, err = environments.NewBuilder().WithAddons(loadImages.Build()).WithExistingCluster(cluster).Build(ctx)
		exitOnErr(err)
	} else {
		fmt.Println("INFO: creating a new kind cluster")

		certManagerBuilder := certmanager.NewBuilder()

		if len(certmanagerVersionStr) != 0 {
			fmt.Printf("INFO: a specific version of certmanager was requested: %s\n", certmanagerVersionStr)
			certmanagerVersion, err := semver.ParseTolerant(certmanagerVersionStr)
			exitOnErr(err)
			certManagerBuilder.WithVersion(certmanagerVersion)
		}

		// create the testing environment and cluster
		env, err = environments.NewBuilder().WithAddons(certManagerBuilder.Build(), loadImages.Build()).Build(ctx)
		exitOnErr(err)

		fmt.Printf("INFO: new kind cluster %s was created\n", env.Cluster().Name())
	}

	if !keepTestCluster {
		addCleanup(func(context.Context) error {
			cleanupLog("cleaning up test environment and cluster %s\n", env.Cluster().Name())
			return env.Cleanup(ctx)
		})
	}

	// deploy the BPFD Operator and revelevant CRDs
	fmt.Println("INFO: deploying bpfd operator to test cluster")
	exitOnErr(clusters.KustomizeDeployForCluster(ctx, env.Cluster(), bpfdKustomize))
	if !keepKustomizeDeploys {
		addCleanup(func(context.Context) error {
			cleanupLog("delete bpfd configmap to cleanup bpfd daemon")
			env.Cluster().Client().CoreV1().ConfigMaps(internal.BpfdNs).Delete(ctx, internal.BpfdConfigName, metav1.DeleteOptions{})
			clusters.DeleteManifestByYAML(ctx, env.Cluster(), bpfdConfigMap)
			waitForBpfdConfigDelete(ctx, env)
			cleanupLog("deleting bpfd namespace")
			return env.Cluster().Client().CoreV1().Namespaces().Delete(ctx, internal.BpfdNs, metav1.DeleteOptions{})
		})
	}

	bpfdClient = bpfdHelpers.GetClientOrDie()
	exitOnErr(waitForBpfdReadiness(ctx, env))

	exit := m.Run()
	// If there's any errors in e2e tests dump diagnostics
	if exit != 0 {
		_, err := env.Cluster().DumpDiagnostics(ctx, "bpfd-e2e-test")
		exitOnErr(err)
	}

	exitOnErr(runCleanup())

	os.Exit(exit)
}

func exitOnErr(err error) {
	if err == nil {
		return
	}

	if cleanupErr := runCleanup(); cleanupErr != nil {
		err = fmt.Errorf("%s; %w", err, cleanupErr)
	}

	if err != nil {
		fmt.Fprint(os.Stderr, err.Error())
		os.Exit(1)
	}
}

func addCleanup(job func(context.Context) error) {
	// prepend so that cleanup runs in reverse order
	cleanup = append([]func(context.Context) error{job}, cleanup...)
}

func cleanupLog(msg string, args ...any) {
	fmt.Printf(fmt.Sprintf("INFO: %s\n", msg), args...)
}

func runCleanup() (cleanupErr error) {
	if len(cleanup) < 1 {
		return
	}

	fmt.Println("INFO: running cleanup jobs")
	for _, job := range cleanup {
		if err := job(ctx); err != nil {
			cleanupErr = fmt.Errorf("%s; %w", err, cleanupErr)
		}
	}
	cleanup = nil
	return
}

func waitForBpfdReadiness(ctx context.Context, env environments.Environment) error {
	for {
		time.Sleep(2 * time.Second)
		select {
		case <-ctx.Done():
			if err := ctx.Err(); err != nil {
				return fmt.Errorf("context completed while waiting for components: %w", err)
			}
			return fmt.Errorf("context completed while waiting for components")
		default:
			fmt.Println("INFO: waiting for bpfd")
			var controlplaneReady, dataplaneReady bool

			controlplane, err := env.Cluster().Client().AppsV1().Deployments(internal.BpfdNs).Get(ctx, internal.BpfdOperatorName, metav1.GetOptions{})
			if err != nil {
				if errors.IsNotFound(err) {
					fmt.Println("INFO: bpfd-operator dep not found yet")
					continue
				}
				return err
			}
			if controlplane.Status.AvailableReplicas > 0 {
				controlplaneReady = true
			}

			dataplane, err := env.Cluster().Client().AppsV1().DaemonSets(internal.BpfdNs).Get(ctx, internal.BpfdDsName, metav1.GetOptions{})
			if err != nil {
				if errors.IsNotFound(err) {
					fmt.Println("INFO: bpfd daemon not found yet")
					continue
				}
				return err
			}
			if dataplane.Status.NumberAvailable > 0 {
				dataplaneReady = true
			}

			if controlplaneReady && dataplaneReady {
				fmt.Println("INFO: bpfd-operator is ready")
				return nil
			}
		}
	}
}

func waitForBpfdConfigDelete(ctx context.Context, env environments.Environment) error {
	for {
		time.Sleep(2 * time.Second)
		select {
		case <-ctx.Done():
			if err := ctx.Err(); err != nil {
				return fmt.Errorf("context completed while waiting for components: %w", err)
			}
			return fmt.Errorf("context completed while waiting for components")
		default:
			fmt.Println("INFO: waiting for bpfd config deletion")

			_, err := env.Cluster().Client().CoreV1().ConfigMaps(internal.BpfdNs).Get(ctx, internal.BpfdConfigName, metav1.GetOptions{})
			if err != nil {
				if errors.IsNotFound(err) {
					fmt.Println("INFO: bpfd configmap deleted successfully")
					return nil
				}
				return err
			}
		}
	}
}
