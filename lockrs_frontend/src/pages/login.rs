use leptos::*;

use crate::components::ui::button::*;
use crate::components::ui::card::*;
use crate::components::ui::link::*;
use crate::components::user::login_form::*;

#[component]
pub fn LoginPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="login-page" class="relative h-full flex-col items-center justify-center lg:max-w-none".to_string()>
            <Button
                class="absolute right-4 top-4 md:right-8 md:top-8".to_string()
                variant=ButtonVariant::Ghost
            >
                <Link href="/register">Register</Link>
            </Button>
            <div class="flex flex-col justify-center items-center h-full">
                <Card>
                    <CardHeader>
                        <CardTitle>Login</CardTitle>
                        <CardDescription>
                            Enter your email and password to login to your account
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <UserLoginForm />
                    </CardContent>
                    <CardFooter>
                        <Link
                            class="w-full text-center".to_string()
                            href="/password-reset"
                        >
                            Forgot password?
                        </Link>
                    </CardFooter>
                </Card>
            </div>
        </div>
    }
}
