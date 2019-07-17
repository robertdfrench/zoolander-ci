output "host" {
  value = {
    domain_name = local.network.dns
    ip_address  = aws_instance.zoolander.public_ip
  }
}

resource "aws_instance" "zoolander" {
  ami             = data.aws_ami.zoolander_latest.id
  instance_type   = "t2.small"
  security_groups = [local.network.security_group]
  key_name        = aws_key_pair.zoolander.key_name

  tags = {
    Name = "zoolander"
  }

  connection {
    host = aws_instance.zoolander.public_ip
  }

  provisioner "remote-exec" {
    # Tell the host what its domain name is 
    inline = [format("hostname -s %s", local.network.dns)]
  }

  provisioner "file" {
    source      = "provisions"
    destination = "/tmp"
  }

  provisioner "remote-exec" {
    inline = ["/usr/gnu/bin/make -C /tmp/provisions"]
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

resource "aws_eip_association" "eip_assoc" {
  instance_id   = aws_instance.zoolander.id
  allocation_id = local.network.ip_allocation
}
