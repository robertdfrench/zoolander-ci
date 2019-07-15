locals {
  network = {
    ip_allocation  = aws_eip.ip.id
    security_group = aws_security_group.zoolander.name
    dns            = aws_route53_record.record.fqdn
  }
}

resource "aws_eip" "ip" {}

resource "aws_route53_record" "record" {
  zone_id = data.aws_route53_zone.parent.id
  name    = "zoolander-ci"
  type    = "A"
  ttl     = "300"
  records = [aws_eip.ip.public_ip]
}

data "aws_route53_zone" "parent" {
  name = local.fully_qualified_parent_zone
}

locals {
  fully_qualified_parent_zone = format("%s.", var.parent_zone)
}

variable "parent_zone" {}

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
