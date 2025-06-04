import { Separator } from "@/components/ui/separator";
import Auditions from "@/features/Auditions";
import Search from "@/features/Search";

export default async function Home() {
	return (
		<main className="bg-background min-h-screen w-screen flex flex-col gap-4 py-6">
			<section className="px-6 flex flex-col gap-2">
				<div className="flex flex-col justify-center h-full">
					<h1 className="text-2xl font-semibold">Welcome to dance auditions</h1>
					<p className="text-sm text-muted-foreground">
						Find your next dance audition
					</p>
				</div>
				<Search />
			</section>
			<Separator className="my-4" />
			<section className="px-6">
				<Auditions />
			</section>
		</main>
	);
}
