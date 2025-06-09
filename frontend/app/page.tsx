import { Separator } from "@/components/ui/separator";
import Auditions from "@/features/Auditions";

export default async function Home() {
	return (
		<main className="bg-background min-h-screen w-screen flex flex-col gap-4 py-6">
			<section className="px-6 flex flex-col gap-2">
				<div className="flex flex-col justify-center h-full">
					<h1 className="text-2xl font-semibold">Dance auditions</h1>
					<p className="text-sm text-muted-foreground">
						Find your next dance audition
					</p>
				</div>
			</section>
			<Separator className="mt-4 mb-2" />
			<section className="px-6">
				<Auditions />
			</section>
		</main>
	);
}
