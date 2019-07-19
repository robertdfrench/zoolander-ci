locals {
  network = {
    security_group = aws_security_group.zoolander.name
    az             = format("%sa", data.aws_region.current.name)
  }
}

data "aws_region" "current" {}

resource "aws_security_group" "zoolander" {
  name        = "zoolander"
  description = "SSH, HTTP, and outbound IP4 for Zoolander CI"

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = [join("/", [data.external.nat_ip.result.ip, 32])]
  }

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "zoolander"
  }
}

data "external" "nat_ip" {
  program = ["bash", "-c", "echo '{'\\\"ip\\\": \\\"`dig -4 @resolver1.opendns.com myip.opendns.com +short`\\\"'}'"]
}
