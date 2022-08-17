provider "aws" {
  region = "us-east-1"
  profile = "default"
}

provider "aws" {
    region = "us-east-1"
    alias = "efgriver_global_devops"
    profile = "devops"
}

provider "aws" {
    region = "ap-northeast-1"
    alias = "efgriver_tokyo_devops"
    profile = "devops"
}
