default:
  just --list

build target image_repo image_tag="latest":
  docker build -f {{target}}-service/Dockerfile -t {{image_repo}}:{{image_tag}} .

create-fake-certificate domain:
  openssl req \
      -x509 -newkey rsa:4096 -sha256 -days 3650 \
      -nodes -keyout {{domain}}.key -out {{domain}}.crt \
      -subj "/CN={{domain}}" -addext "subjectAltName=DNS:{{domain}},DNS:*.{{domain}},IP:0.0.0.0"

register-fake-certificate domain:
  aws acm import-certificate \
    --certificate "fileb://{{domain}}.crt" \
    --private-key "fileb://{{domain}}.key" \
    --query CertificateArn --output text \
  | cat

greet target name:
  #!/bin/sh -eux
  domain="$( kubectl get ing --selector app.kubernetes.io/instance={{target}}-service -ojsonpath="{ .items[0].status.loadBalancer.ingress[0].hostname }" )"
  rpc_name="$( echo "{{target}}" | sed "s/./\U&/" )Rpc"
  grpcurl -insecure -import-path ./proto -proto service/{{target}}.proto -d "{ \"name\": \"{{name}}\" }" $domain:443 service.{{target}}.$rpc_name/Greet
