output "host" {
  value = {
    domain_name = local.host.dns
  }
}

locals {
  host = {
    dns = aws_route53_record.record.fqdn
  }
}

resource "aws_route53_record" "record" {
  zone_id = data.aws_route53_zone.parent.id
  name    = "zoolander-ci"
  type    = "A"
  ttl     = "300"
  records = [aws_instance.zoolander.public_ip]
}

data "aws_route53_zone" "parent" {
  name = format("%s.", var.parent_zone)
}

variable "parent_zone" {}

resource "aws_instance" "zoolander" {
  ami             = data.aws_ami.zoolander_latest.id
  instance_type   = "t2.small"
  security_groups = [local.network.security_group]
  key_name        = aws_key_pair.zoolander.key_name

  tags = {
    Name = "zoolander"
  }
}

data "aws_ami" "zoolander_latest" {
  most_recent = true
  owners      = ["self"]
  name_regex  = "^zoolander-*"
}

resource "aws_key_pair" "zoolander" {
  key_name   = "zoolander"
  public_key = file(pathexpand("~/.ssh/id_rsa.pub"))
}

resource "aws_volume_attachment" "left" {
  device_name = "/dev/sdf" # Required Linuxism... Just ignore
  volume_id   = local.storage.left
  instance_id = aws_instance.zoolander.id
}

resource "aws_volume_attachment" "right" {
  device_name = "/dev/sdg" # Required Linuxism... Just ignore
  volume_id   = local.storage.right
  instance_id = aws_instance.zoolander.id
}
