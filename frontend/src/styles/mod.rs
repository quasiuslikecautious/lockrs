use stylist::{style, Style};

// #F15025 #FFFFFF #E6E8E6 #CED0CE #191919

pub fn form_styles() -> Style {
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
                width: 360px;
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
            
            /* Container specific attr that should apply to children */
            #form-container {
                padding: 40px;
            }

            input {
                margin-bottom: 10px;
            }

            input[type=text],input[type=password] {
                height: 30px;
                width: 100%;
                padding: 10px 5px;

                border-radius: 5px;

                font-size: large;
            }
        "#
    ).expect("Failed to mount style");
}

pub fn confirm_button_pair() -> Style {
    return style!(
        r#"
            display: flex;
            width: 100%;

            align-self: flex-end;

            flex-flow: row wrap;
            align-items: center;
            justify-content: space-around;

            button.secondary {
                height: 35px;
                width: 100px;

                border: 1px solid #F15025;
                border-radius: 5px;

                background-color: #FFFFFF;
            }

            button.secondary:hover {
                background-color: #E0E0E0;
            }

            button.secondary:active {
                background-color: #C2C2C2;
                transition: 0.2s;
            }

            button.secondary p {
                color: #F15025;
            }
        "#
    ).expect("Failed to mount style");
}

