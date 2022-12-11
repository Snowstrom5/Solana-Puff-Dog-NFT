variable "aws_region" {
  default = "us-east-2"
}

variable "aws_availability_zone" {
  default = "us-east-2a"
}

variable "aws_key_name" {
  default = "climatenews_app"
}

variable "aws_pem_file" {
  default = "~/.ssh/climatenews_app.pem"
}

variable "aws_instance_type" {
  default = "t2.micro"
}

variable "aws_base_ami" {
  default = "ami-0283a57753b18025b"
}

variable "base_domain_name" {
  default = "climatenews.app"
}

variable "terraform_state_bucket" {
  default = "climate-news-terraform-state"
}

variable "terraform_state_lock_table" {
  default = "climate-news-terraform-state-lock-table"
}


variable "MY_IP_ADDRESS" {
    type        = string
    description = "An IP that is allowed to ssh into the host. It is set as an env variable. TF_VAR_MY_IP_ADDRESS"
}