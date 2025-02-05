#!/usr/bin/env bash

# Copyright 2022 The Kubernetes Authors.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

set -o errexit
set -o nounset
set -o pipefail

thisyear=`date +"%Y"`

mkdir -p release/

## Location to install dependencies to
LOCALBIN=$(pwd)/bin

## Tool Binaries
KUSTOMIZE=${LOCALBIN}/kustomize

# Generate all install yaml's

## 1. bpfd CRD install

# Make clean files with boilerplate
cat hack/boilerplate.sh.txt > release/bpfd-crds-install-v${VERSION}.yaml
sed -i "s/YEAR/$thisyear/g" release/bpfd-crds-install-v${VERSION}.yaml
cat << EOF >> release/bpfd-crds-install-v${VERSION}.yaml
#
# bpfd Kubernetes API install
#
EOF

for file in `ls config/crd/bases/bpfd*.yaml`
do
    echo "---" >> release/bpfd-crds-install-v${VERSION}.yaml
    echo "#" >> release/bpfd-crds-install-v${VERSION}.yaml
    echo "# $file" >> release/bpfd-crds-install-v${VERSION}.yaml
    echo "#" >> release/bpfd-crds-install-v${VERSION}.yaml
    cat $file >> release/bpfd-crds-install-v${VERSION}.yaml
done

echo "Generated:" release/bpfd-crds-install-v${VERSION}.yaml

## 2. bpfd-operator install yaml

$(cd ./config/bpfd-operator-deployment && ${KUSTOMIZE} edit set image quay.io/bpfd/bpfd-operator=quay.io/bpfd/bpfd-operator:v${VERSION})
${KUSTOMIZE} build ./config/default > release/bpfd-operator-install-v${VERSION}.yaml
### replace configmap :latest images with :v${VERSION}
sed -i "s/quay.io\/bpfd\/bpfd-agent:latest/quay.io\/bpfd\/bpfd-agent:v${VERSION}/g" release/bpfd-operator-install-v${VERSION}.yaml
sed -i "s/quay.io\/bpfd\/bpfd:latest/quay.io\/bpfd\/bpfd:v${VERSION}/g" release/bpfd-operator-install-v${VERSION}.yaml

echo "Generated:" release/bpfd-operator-install-v${VERSION}.yaml

## 3. examples install yamls

### XDP
${KUSTOMIZE} build ../examples/config/v${VERSION}/go-xdp-counter > release/go-xdp-counter-install-v${VERSION}.yaml
echo "Generated:" go-xdp-counter-install-v${VERSION}.yaml
### TC
${KUSTOMIZE} build ../examples/config/v${VERSION}/go-tc-counter > release/go-tc-counter-install-v${VERSION}.yaml
echo "Generated:" go-tc-counter-install-v${VERSION}.yaml
### TRACEPOINT
${KUSTOMIZE} build ../examples/config/v${VERSION}/go-tracepoint-counter > release/go-tracepoint-counter-install-v${VERSION}.yaml
echo "Generated:" go-tracepoint-counter-install-v${VERSION}.yaml
