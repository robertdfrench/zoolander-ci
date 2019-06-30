# Base Image
[OmniOS][1] Base Image with extra packages installed to support the zoolander
environment.

We do some shenanigans here to grab the id of the newly minted AMI and report
it up the stack: once packer has completed, we apply a small terraform plan that
simply looks up the AMI by name and returns its id. But the plan itself does not
result in any infrastructure changes.

See the [Makefile](Makefile) for more details.

[1]: https://omniosce.org/
