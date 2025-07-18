pub fn template_01(email_add: String, email_subj: String, email_msg: String) -> String {
    let style = String::from(
        "<style>
        body {
            font-family: Arial, sans-serif;
            background-color: #f9f9f9;
            margin: 0;
            padding: 0;
        }
        .container {
            width: 100%;
            max-width: 600px;
            margin: 0 auto;
            background-color: #ffffff;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        .header {
            text-align: center;
            margin-bottom: 20px;
        }
        .header h1 {
            font-size: 24px;
            color: #333333;
        }
        .content {
            font-size: 16px;
            line-height: 1.6;
            color: #333333;
        }
        .content p {
            margin: 10px 0;
        }
        .footer {
            text-align: center;
            font-size: 14px;
            color: #777777;
            margin-top: 20px;
        }
        .footer a {
            color: #007BFF;
            text-decoration: none;
        }
    </style>",
    );

    let head_tag = format!(
        "<!DOCTYPE html>
        <html lang='en'>
        <head>
            <meta charset='UTF-8'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <title>Contact Us</title>
            {}
        </head>",
        style
    );

    format!(
        "{}
            <body>
                <div class='container'>
                    <div class='header'>
                        <h1>Contact Us Message</h1>
                    </div>

                    <div class='content'>
                        <p><strong>Email:</strong> {}</p>
                        <p><strong>Subject:</strong> {}</p>
                        <p><strong>Message:</strong></p>
                        <p>{}</p>
                    </div>
                </div>

            </body>
            </html>
            ",
        head_tag, email_add, email_subj, email_msg
    )
}

pub fn template_02(
    email_fname: String,
    email_add: String,
    email_subj: String,
    email_msg: String,
) -> String {
    let style = String::from(
        "<style>
        body {
            font-family: Arial, sans-serif;
            background-color: #f9f9f9;
            margin: 0;
            padding: 0;
        }
        .container {
            width: 100%;
            max-width: 600px;
            margin: 0 auto;
            background-color: #ffffff;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        .header {
            text-align: center;
            margin-bottom: 20px;
        }
        .header h1 {
            font-size: 24px;
            color: #333333;
        }
        .content {
            font-size: 16px;
            line-height: 1.6;
            color: #333333;
        }
        .content p {
            margin: 10px 0;
        }
        .footer {
            text-align: center;
            font-size: 14px;
            color: #777777;
            margin-top: 20px;
        }
        .footer a {
            color: #007BFF;
            text-decoration: none;
        }
    </style>",
    );

    let head_tag = format!(
        "<!DOCTYPE html>
        <html lang='en'>
        <head>
            <meta charset='UTF-8'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <title>Contact Us</title>
            {}
        </head>",
        style
    );

    format!(
        "{}
            <body>
                <div class='container'>
                    <div class='header'>
                        <h1>Contact Us Message</h1>
                    </div>

                    <div class='content'>
                        <p><strong>Fullname:</strong> {}</p>
                        <p><strong>Email:</strong> {}</p>
                        <p><strong>Subject:</strong> {}</p>
                        <p><strong>Message:</strong></p>
                        <p>{}</p>
                    </div>
                </div>

            </body>
            </html>
            ",
        head_tag, email_fname, email_add, email_subj, email_msg
    )
}
