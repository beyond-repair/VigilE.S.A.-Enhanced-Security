module "vigil_security" {
  source = "terraform-aws-modules/vpc/aws"
  
  hsm_arn = var.hsm_arn
  sgx_enabled = true
  
  network_policies = {
    allow_ebpf = true
    zero_trust = true
  }
}

resource "aws_kms_key" "vigil_hsm" {
  description = "VigilE.S.A. HSM Key"
  policy = data.aws_iam_policy_document.hsm_policy.json
}
