use leptos::*;

// <p>"This application is built entirely using Rust, leveraging the powerful and efficient Leptos framework for the frontend and Axum for the backend. With a focus on security and performance, it will provide comprehensive authentication and authorization features to meet your application's needs."</p>
//                <p>"Key features:"</p>
//                <ul>
//                     <li>"OAuth 2.0 Support: Our application fully supports OAuth 2.0, including popular flows such as client credentials, PKCE (Proof Key for Code Exchange), device code, and OIDC (OpenID Connect). This allows you to integrate with various external authentication providers seamlessly."</li>
//                     <li>"In-House User Authentication: In addition to external authentication providers, we offer a robust in-house user authentication mechanism. You can create and manage user accounts directly within our application, providing a secure and customized authentication experience for your users."</li>
//                     <li>"Secure and Reliable: Rust's strong focus on memory safety and thread safety ensures that our application is resistant to common security vulnerabilities. We follow industry best practices to safeguard user credentials and protect sensitive information throughout the authentication and authorization process."</li>
//                     <li>"Efficient Frontend: Leptos, our frontend framework of choice, provides a modern and responsive user interface. With its lightweight nature and minimal footprint, it ensures a smooth and fast user experience while keeping resource usage to a minimum."</li>
//                     <li>"Scalable Backend: Axum, our backend framework, is designed for high-performance applications. It enables concurrent processing, making it ideal for handling large numbers of authentication requests. Whether you have a small user base or a rapidly growing one, our application can scale to meet your needs."</li>
//                     <li>"Developer-Friendly: Rust's strong type system and expressive syntax make it a joy to work with. Our application follows best practices and provides clean and maintainable code, allowing developers to easily extend and customize the authentication and authorization features as needed."</li>
//                </ul>
//                <p>"We are excited to offer you a reliable and secure solution for all your authentication and authorization requirements. Whether you are building a web application, mobile app, or any other software that requires user authentication, our Rust-based application has got you covered."</p>
//                <p>"Get started today and empower your users with a seamless and secure authentication experience. Contact us to learn more and integrate our authentication and authorization application into your project."</p>

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="home-page" class="flex flex-col">
            <div id="spacer" class="py-32" />
            <div id="home-title" class="mb-8 py-1 text-center">
                <h2 class="text-3xl font-bold">
                    "Harder. Better. Faster. Stronger."
                </h2>
            </div>
        </div>

        <form>
            <button>
            </button>
        </form>
    }
}
