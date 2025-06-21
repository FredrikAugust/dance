import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import Auditions from "@/features/Auditions";
import { cookies } from "next/headers";
import Link from "next/link";

export default async function Home() {
  const cookieStore = await cookies()

  const authenticated = cookieStore.get("session-id");

  return (
    <main className="bg-background min-h-screen w-screen flex flex-col gap-4 py-6">
      <section className="px-6 flex gap-2 items-center justify-between">
        <div className="flex flex-col justify-center h-full">
          <h1 className="text-2xl font-semibold">Dance auditions</h1>
          <p className="text-sm text-muted-foreground">
            Find your next dance audition
          </p>
        </div>

        {!authenticated && (
          <Link href="/login">
            <Button>Sign in</Button>
          </Link>
        )}
      </section>
      <Separator className="mt-4 mb-2" />
      <section className="px-6">
        <Auditions />
      </section>
    </main>
  );
}
