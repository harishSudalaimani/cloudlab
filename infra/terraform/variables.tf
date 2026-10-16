variable "aws_region" { type = string default = "ap-south-1" }
variable "domain_name" { type = string default = "kloudlab.in" }
variable "api_instance_type" { type = string default = "t4g.nano" }
variable "api_image" { type = string default = "" description = "Optional pre-built API image URI. No instance is created until applied." }
