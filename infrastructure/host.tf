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
    inline = ["mkdir -p /etc/dehydrated"]
  }

  provisioner "file" {
    source      = "dehydrated.config"
    destination = "/etc/dehydrated/config"
  }

  provisioner "remote-exec" {
    inline = ["dehydrated --register --accept-terms"]
  }

  provisioner "file" {
    source      = "nginx.conf"
    destination = "/etc/opt/ooce/nginx-1.16/nginx.conf"
  }

  provisioner "remote-exec" {
    inline = [
      format("hostname -s %s", local.network.dns),
      "svcadm enable svc:/network/http",
      "git init zoolander"
    ]
  }

  provisioner "file" {
    source      = "post-receive-hook.sh"
    destination = "zoolander/.git/hooks/post-receive"
  }

  provisioner "remote-exec" {
    inline = [
      "chmod +x zoolander/.git/hooks/post-receive",
      "git config --system receive.denyCurrentBranch ignore",
      "useradd -b /export -m -s /bin/bash derek"
    ]
  }

  provisioner "file" {
    source      = "derek_profile.bash"
    destination = "/export/derek/.profile"
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
