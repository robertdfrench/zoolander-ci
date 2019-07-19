locals {
  storage = {
    attachment = aws_volume_attachment.storage
  }
}

resource "aws_ebs_volume" "storage" {
  availability_zone = local.network.az
  size              = 1

  tags = {
    Name = "zoolander"
  }
}

resource "aws_volume_attachment" "storage" {
  device_name  = "/dev/sdf" # Required Linuxism... Just ignore
  volume_id    = aws_ebs_volume.storage.id
  instance_id  = local.host.id
  force_detach = true

  connection {
    host = local.host.dns
  }

  provisioner "file" {
    source      = "provisions"
    destination = "/tmp"
  }

  provisioner "remote-exec" {
    # Connect storage, DNS, and begin provisioning
    inline = [
      "zpool create -m /persistent persistent c1t5d0",
      format("hostname -s %s", local.host.dns),
      "/usr/gnu/bin/make -C /tmp/provisions"
    ]
  }

  provisioner "remote-exec" {
    when = "destroy"

    inline = [
      "zpool destroy -f persistent"
    ]
  }
}
