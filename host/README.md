# Application Host Instance
An EC2 instance configured to server the zoolander application, using the image
defined in [image/](image/).

An EC2 Security Group (a firewall, or close enough) is configured to allow
HTTP(S) access globally, but restrict SSH access to the public IP4 address to
of the host issuing the terraform commands. Should this change, simply re-apply
the terraform plan to update your security groups. This may or may not blow away
your zoolander host.

Services are configured in this step, as well as a root-owned git repo which
will enable push-on-deploy for the application itself. This allows us to beat
the chicken-and-egg problem of "who delivers the delivery system?".

See the [Makefile](Makefile) for more details.
