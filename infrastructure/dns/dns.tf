output "ip" {
  value = {
    id  = aws_eip.ip.id
    ip  = aws_eip.ip.public_ip
    dns = aws_route53_record.record.fqdn
  }
}

resource "aws_eip" "ip" {
  vpc = false
}

resource "aws_route53_record" "record" {
	zone_id = data.aws_route53_zone.parent.id
	name = "zoolander-ci"
	type = "A"
	ttl = "300"
	records = aws_eip.ip.public_ip
}

data "aws_route53_zone" "parent" {
	name = local.fully_qualified_parent_zone
}

locals {
	fully_qualified_parent_zone = format("%s.", var.parent_zone.value)
}

variable "parent_zone" {}
