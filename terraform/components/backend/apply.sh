#!/bin/sh

terraform validate
terraform workspace new test
terraform workspace select test
terraform plan -out=terraform.plan
terraform apply "terraform.plan"