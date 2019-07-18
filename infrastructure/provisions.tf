resource "null_resource" "provisions" {
  connection {
    host = local.host.dns
  }

  provisioner "remote-exec" {
    # Tell the host what its domain name is 
    inline = [format("hostname -s %s", local.host.dns)]
  }

  provisioner "file" {
    source      = "provisions"
    destination = "/tmp"
  }

  provisioner "remote-exec" {
    inline = ["/usr/gnu/bin/make -C /tmp/provisions"]
  }
}
