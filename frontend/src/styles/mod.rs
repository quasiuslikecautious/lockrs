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
                width: 100%;
                margin-top: auto;

                align-self: flex-end;

                border: none;
                border-radius: 5px;

                background-color: #F15025;

                transition-duration: 0.2s;
                transition-timing-function: ease-in-out;
                transition-property: background-color;
            }

            button:hover {
                background-color: #E63D0F;
            }

            button:active {
                background-color: #862409;
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
                width: 300px;
                position: relative;
                margin-left: auto;
                margin-right: auto;

                display: flex;
                flex-flow: column wrap;
                align-items: center;
                gap: 10px;
                
                border: 10px solid #EBEBEB;
                border-radius: 8px;

                transition-duration: 1s;
                transition-timing-function: ease-in-out;
                transition-property: transform;
            }

            .container:hover, .container:focus-within {
                transform: translateY(50px); 
            }
            
            /* Container specific attr that should apply to children */
            #form-container {
                margin-top: 50px;
                padding: 20px 40px;
                background-color: #FFFFFF;
                box-shadow: 10px 10px 10px rgba(0, 0, 0, 0.2);
            }

            #cutout {
                width: 100px;
                height: 15px;

                border: 3px solid #EBEBEB;
                border-radius: 15px;
                box-shadow: 2px 2px 2px 2px rgba(0, 0, 0, 0.2) inset;
            }

            #lanyard-front {
                width: 200px;

                transform: rotate(-99.2deg);
                position: absolute;
                top: -100px;
                left: 69px;

                border-bottom: 60px solid #F15025;
                border-left: 10px solid transparent;

                box-shadow: 2px 2px 2px rgba(0, 0, 0, 0.1);
            }

            #lanyard-back {
                width: 160px;

                transform: rotate(99.2deg);
                position: absolute;
                top: -119px;
                left: 125px;

                border-bottom: 60px solid #C0330C;
                border-right: 10px solid transparent;
                
            }

            input, textarea {
                border: 1px solid #CED0CE;

                transition-duration: 0.1s;
                transition-timing-function: ease-in-out;
                transition-property: border;
            }

            textarea {
                min-height: 120px;
                width: 100%;
                padding: 10px;

                resize: none;
                maxlength: 200;
            }

            input:focus, textarea:focus {
                outline: none;
                border: 2px solid #25C6F1;
            }

            input.invalid, textarea.invalid {
                border: 1px solid #F12560 !important;
            }

            input.invalid:focus, textarea.invalid:focus {
                border: 2px solid #D30D45 !important;
            }

            input[type=text], input[type=password] {
                height: 30px;
                padding: 24px 10px;
            }

            input[type=text], input[type=password], textarea {
                width: 100%;
                box-sizing: border-box;
                font-size: 16px;

                border-radius: 5px;
            }

            .input-container {
                position: relative;
                margin-bottom: 10px;
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

            input:focus + .input-hint, textarea:focus + .input-hint {
                padding: 0 4px;

                font-size: 12px;
                color: #25C6F1;

                transform: translateX(-4px) translateY(-23px);
            }

            input:not(:placeholder-shown) + .input-hint, textarea:not(:placeholder-shown) + .input-hint {
                padding: 0 4px;

                font-size: 12px;

                transform: translateX(-4px) translateY(-22px);
            }

            input.invalid + .input-hint, textarea.invalid + .input-hint {
                color: #F12560 !important;
            }

            #char-counter {
                display: flex;
                flex-flow: row wrap;
                justify-content: flex-end;

                font-style: italic;
                font-size: 0.875rem;
            }
        "#
    ).expect("Failed to mount style");
}

pub fn button_pair() -> Style {
    return style!(
        r#"
            display: flex;
            width: 100%;
            margin-top: auto;

            align-self: flex-end;

            flex-flow: row wrap;
            align-items: center;
            justify-content: space-around;
            gap: 10px;

            button.secondary {
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

