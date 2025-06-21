"use client";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { API_URL } from "@/lib/utils";

export default function Login() {
  const login = async (formData: FormData) => {
    const response = await fetch(`${API_URL}/login`, {
      body: JSON.stringify({
        email: formData.get("email"),
        password: formData.get("password")
      }),
      headers: {
        "Content-Type": "application/json"
      },
      method: "POST"
    })

    const clientCookies = await cookies();
    clientCookies.set("test", "abc");
    console.debug(await response.text())
  }

  return (
    <main className="min-h-screen w-screen flex items-center justify-center">
      <div className="w-[80%] border border-gray-300 rounded-md py-2 px-4 flex gap-2 flex-col">
        <h3 className="text-lg font-medium text-gray-700">Login</h3>

        <form action={login} className="flex flex-col gap-2">
          <div>
            <label htmlFor="email" className="block mb-2 text-sm font-medium text-gray-700">
              Email
            </label>
            <Input type="email" name="email" id="email" />
          </div>
          <div>
            <label htmlFor="password" className="block mb-2 text-sm font-medium text-gray-700">
              Password
            </label>
            <Input autoComplete="current-password" type="password" name="password" id="password" />
          </div>

          <Button type="submit" className="mt-4">Sign in</Button>
        </form>

        <div>
        </div>
      </div>
    </main >
  )
}
