locals {
    function_name = "Trello-Reminder"
}

resource "aws_lambda_function" "function" {
    function_name = local.function_name
    description = "Monthly Trello Reminder"
    role = aws_iam_role.execution_role.arn

    filename = "${path.module}/package.zip"
    source_code_hash = filebase64sha256("${path.module}/package.zip")
    handler = "hello.handler"
    runtime = "provided.al2"

    environment {
        variables = {
            RUST_BACKTRACE = 1
        }
    }

    depends_on = [aws_iam_role.execution_role, aws_cloudwatch_log_group.log_group]
}

resource "aws_cloudwatch_event_rule" "execution_trigger" {
    name = "${local.function_name}-ExecutionTrigger"
    schedule_expression = "cron(0 0 1 * ? *)"
}

resource "aws_cloudwatch_event_target" "execution_trigger_target" {
    rule = aws_cloudwatch_event_rule.execution_trigger.name
    arn = aws_lambda_function.function.arn
}

resource "aws_iam_role" "execution_role" {
    name = "${local.function_name}-ExecutionRole"
    path = "/service-role/${local.function_name}/"

    assume_role_policy = jsonencode({
        Version = "2012-10-17",
        Statement = [
            {
                Effect = "Allow",
                Action = "sts:AssumeRole",
                Principal = { Service = "lambda.amazonaws.com" }
            }
        ]
    })
}

resource "aws_cloudwatch_log_group" "log_group" {
    name = "/aws/lambda/${local.function_name}"
    retention_in_days = 1
}

resource "aws_iam_policy" "logging_policy" {
    name = "${local.function_name}-LoggingPolicy"
    path = "/services/${local.function_name}/"
    policy = jsonencode({
        Version = "2012-10-17",
        Statement = [
            {
                Effect = "Allow",
                Action = ["logs:CreateLogStream", "logs:PutLogEvents"],
                Resource = "${aws_cloudwatch_log_group.log_group.arn}:*"
            }
        ]
    })
}

resource "aws_iam_role_policy_attachment" "execution_role_logging_policy" {
    role = aws_iam_role.execution_role.name
    policy_arn = aws_iam_policy.logging_policy.arn
}

resource "aws_iam_policy" "secrets_read_policy" {
    name = "${local.function_name}-SecretsReadPolicy"
    path = "/services/${local.function_name}/"
    policy = jsonencode({
        Version = "2012-10-17",
        Statement = [
            {
                Effect = "Allow",
                Action = "secretsmanager:GetSecretValue",
                Resource = data.aws_secretsmanager_secret.secrets.arn
            }
        ]
    })
}

resource "aws_iam_role_policy_attachment" "execution_role_secrets_read_policy" {
    role = aws_iam_role.execution_role.name
    policy_arn = aws_iam_policy.secrets_read_policy.arn
}

data "aws_secretsmanager_secret" "secrets" {
    name = "trello-reminder"
}
