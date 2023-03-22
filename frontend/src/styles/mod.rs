use stylist::{style, Style};

// #F15025 #FFFFFF #E6E8E6 #CED0CE #191919

pub fn get_device_styles() -> Style {
    return style!(
        r#"
            height: 100%;
            width: 100%;

            font-family: Arial, Helvetica, sans-serif;

            button {
                height: 35px;
                width: 100px;

                align-self: flex-end;

                border: none;
                border-radius: 5px;

                background-color: #F15025;
            }

            button:hover {
                background-color: #E63D0F;
            }

            button:active {
                background-color: #C0330C;
                transition: 0.2s;
            }

            button p {
                color: #FFFFFF;
                margin: 0;
            }

            form {
                width: 90%;
            }
            
            h1,h2,h3,h4,h5,h6 {
                margin: 0;
                font-weight: normal;
            }

            .container {
                height: 500px;
                width: 400px;
                position: relative;
                margin-left: auto;
                margin-right: auto;

                display: flex;
                flex-flow: column wrap;
                align-items: center;
                gap: 20px;
                
                border: 1px solid #CED0CE;
                border-radius: 8px;
            }
            
            #device-code-container {
                padding: 40px;
            }

            input[type=text] {
                height: 30px;
                width: 100%;
                padding: 10px 5px;

                border-radius: 5px;

                font-size: large;
            }
        "#
    ).expect("Failed to mount style");
}

pub fn get_client_registration_styles() -> Style {
    return style!(
        r#" 
            height: 100%;
            width: 100%;

            font-family: Arial, Helvetica, sans-serif;

            button {
                height: 35px;
                width: 100px;

                align-self: flex-end;

                border: none;
                border-radius: 5px;

                background-color: #F15025;
            }

            button:hover {
                background-color: #E63D0F;
            }

            button:active {
                background-color: #C0330C;
                transition: 0.2s;
            }

            button p {
                color: #FFFFFF;
                margin: 0;
            }

            form {
                width: 90%;
            }
            
            h1,h2,h3,h4,h5,h6 {
                margin: 0;
                font-weight: normal;
            }

            .container {
                height: 500px;
                width: 400px;
                position: relative;
                margin-left: auto;
                margin-right: auto;

                display: flex;
                flex-flow: column wrap;
                align-items: center;
                gap: 20px;
                
                border: 1px solid #CED0CE;
                border-radius: 8px;
            }
            
            #client-registration-container {
                padding: 40px;
            }

            input[type=text] {
                height: 30px;
                width: 100%;
                padding: 10px 5px;
                margin: 5px 0;

                border-radius: 5px;

                font-size: large;
            }
        "#
    ).expect("Failed to mount style");
}

pub fn get_login_styles() -> Style {
    return style!(
        r#" 
            height: 100%;
            width: 100%;

            font-family: Arial, Helvetica, sans-serif;

            button {
                height: 35px;
                width: 100px;

                align-self: flex-end;

                border: none;
                border-radius: 5px;

                background-color: #F15025;
            }

            button:hover {
                background-color: #E63D0F;
            }

            button:active {
                background-color: #C0330C;
                transition: 0.2s;
            }

            button p {
                color: #FFFFFF;
                margin: 0;
            }

            form {
                width: 90%;
            }
            
            h1,h2,h3,h4,h5,h6 {
                margin: 0;
                font-weight: normal;
            }

            .container {
                height: 500px;
                width: 400px;
                position: relative;
                margin-left: auto;
                margin-right: auto;

                display: flex;
                flex-flow: column wrap;
                align-items: center;
                gap: 20px;
                
                border: 1px solid #CED0CE;
                border-radius: 8px;
            }
            
            #login-container {
                padding: 40px;
            }

            input[type=text],input[type=password] {
                height: 30px;
                width: 100%;
                padding: 10px 5px;
                margin: 5px 0;

                border-radius: 5px;

                font-size: large;
            }
        "#
    ).expect("Failed to mount style");
}
