let GHA =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/schema.dhall

let actions/checkout =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions/checkout.dhall

let actions-rs/toolchain =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions-rs/toolchain.dhall

let actions-rs/cargo =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions-rs/cargo.dhall

let aws-actions/configure-aws-credentials =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/aws-actions/configure-aws-credentials.dhall

let secrets =
      { AWS_ACCESS_KEY_ID = GHA.mkExpression "secrets.AWS_ACCESS_KEY_ID"
      , AWS_ACCESS_KEY_SECRET = GHA.mkExpression "secrets.AWS_ACCESS_KEY_SECRET"
      }

in  GHA.Workflow::{
    , name = Some "Deployment"
    , on =
        GHA.On.Detailed
          GHA.OnDetails::{
          , push = Some GHA.OnPush::{ branches = Some [ "main" ] }
          }
    , jobs = toMap
        { deployment = GHA.Job::{
          , name = Some "deployment"
          , environment = Some "prod"
          , runs-on = GHA.RunnerPlatform.ubuntu-latest
          , steps =
            [ actions/checkout.stepv3 actions/checkout.Params::{=}
            , actions-rs/toolchain.step
                actions-rs/toolchain.Params::{
                , toolchain = Some "stable"
                , profile = Some "minimal"
                }
            , GHA.Step::{
              , name = "Setup cargo-lambda"
              , run = Some "pip3 install cargo-lambda"
              }
            ,     actions-rs/cargo.step
                    actions-rs/cargo.Params::{
                    , command = "lambda"
                    , args = Some "build --release --output-format zip"
                    }
              //  { name = "Build function code" }
            , GHA.Step::{
              , name = "Setup Terraform"
              , uses = Some "hashicorp/setup-terraform@v2"
              }
            , aws-actions/configure-aws-credentials.step
                aws-actions/configure-aws-credentials.Params::{
                , awsRegion = "ap-northeast-1"
                , awsAccessKeyID = Some secrets.AWS_ACCESS_KEY_ID
                , awsSecretAccessKey = Some secrets.AWS_ACCESS_KEY_SECRET
                }
            , GHA.Step::{
              , name = "Deploy Infrastructure"
              , run = Some
                  "terraform init -input=false && terraform apply -auto-approve"
              }
            ]
          }
        }
    }
