use leptos::*;

use crate::components::ui::button::*;
use crate::components::ui::card::*;
use crate::components::ui::link::*;
use crate::components::user::register_form::*;

#[component]
pub fn RegisterPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="register-page" class="relative h-full flex-col items-center justify-center">
            <Button
                class="absolute right-4 top-4 md:right-8 md:top-8".to_string()
                variant=ButtonVariant::Ghost
            >
                <Link href="/login">Login</Link>
            </Button>
            <div class="flex flex-col justify-center items-center h-full">
                <Card>
                    <CardHeader>
                        <CardTitle>Register</CardTitle>
                        <CardDescription>
                            Enter an email and password to create an account
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <UserRegisterForm />
                    </CardContent>
                    <CardFooter>
                        <p class="w-full text-center text-muted-foreground">
                            By clicking continue, you agree to our<br />
                            <Link href="/legal#terms-of-service">Terms of Service</Link>
                            {" "}and{" "}
                            <Link href="/legal#privacy">Privacy Policy</Link>
                        </p>
                    </CardFooter>
                </Card>
            </div>
        </div>
    }
}
