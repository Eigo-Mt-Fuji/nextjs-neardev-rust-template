#!/bin/sh

terraform validate
terraform workspace select prd
terraform plan -out=terraform.plan
terraform apply "terraform.plan"