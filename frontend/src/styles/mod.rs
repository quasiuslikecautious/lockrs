use stylist::{style, Style};

// pallete    #F15025 #FFFFFF #E6E8E6 #CED0CE #191919
// compliment #F15025 #25C6F1
// analgous   #F15025 #F12560 #F1B625
// triadic    #F15025 #25F150 #5025F1
// tetradic   #F15025 #60F125 #25C6F1 #B625F1

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

                transition-duration: 0.2s;
                transition-timing-function: ease-in-out;
                transition-property: background-color border;
            }

            button:hover {
                background-color: #E63D0F;
            }

            button:active {
                background-color: #C0330C;
            }

            button p {
                color: #FFFFFF;
                margin: 0;
            }

            form {
                width: 100%;
                margin-top: 20px;
            }
            
            h1,h2,h3,h4,h5,h6 {
                margin: 0;
                font-weight: normal;
            }

            .container {
                min-height: 500px;
                width: 360px;
                position: relative;
                margin-left: auto;
                margin-right: auto;

                display: flex;
                flex-flow: column wrap;
                align-items: center;
                gap: 10px;
                
                border: 1px solid #CED0CE;
                border-radius: 8px;
            }
            
            /* Container specific attr that should apply to children */
            #form-container {
                padding: 20px 40px;
            }

            input {
                margin-bottom: 10px;

                border: 1px solid #CED0CE;

                transition-duration: 0.1s;
                transition-timing-function: ease-in-out;
                transition-property: border;
            }

            input:focus {
                outline: none;
                border: 2px solid #25C6F1;
            }

            input.invalid {
                border: 1px solid #F12560 !important;
            }

            input.invalid:focus {
                border: 2px solid #D30D45 !important;
            }

            input[type=text],input[type=password] {
                height: 30px;
                width: 100%;
                padding: 24px 10px;
                box-sizing: border-box;
                font-size: 16px;

                border-radius: 5px;
            }

            .input-container {
                position: relative;
            }

            .input-hint {
                position: absolute;
                top: 16px;
                left: 12px;

                font-size: 16px;
                color: #787D78;

                background-color: #FFFFFF;

                transition-duration: 0.2s;
                transition-timing-function: cubic;
                transition-property: padding position color font-size;
            }

            input:focus + .input-hint {
                padding: 0 4px;

                font-size: 12px;
                color: #25C6F1;

                transform: translateX(-4px) translateY(-23px);
            }

            input:not(:placeholder-shown) + .input-hint {
                padding: 0 4px;

                font-size: 12px;

                transform: translateX(-4px) translateY(-22px);
            }

            input.invalid + .input-hint {
                color: #F12560 !important;
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

