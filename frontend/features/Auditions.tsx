"use server";

import { Button } from "@/components/ui/button";
import {
	Card,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { API_URL } from "@/lib/utils";
import { format } from "date-fns";
import { Suspense } from "react";

type Opportunity = {
	id: string;
	location: string;
	start_time: string;
	end_time: string | null;
	image_urls: string[];
	description: string;
	application_url: string | null;
};

async function getAuditions(): Promise<Opportunity[]> {
	const response = await fetch(`${API_URL}/opportunities`);
	const data = await response.json();
	return data;
}

export default async function Auditions() {
	const auditionsPromise = getAuditions();
	return (
		<Suspense fallback={<div>Loading...</div>}>
			<AuditionsList auditionsPromise={auditionsPromise} />
		</Suspense>
	);
}

async function AuditionsList({
	auditionsPromise,
}: {
	auditionsPromise: Promise<Opportunity[]>;
}) {
	const auditions = await auditionsPromise;
	return (
		<div className="flex flex-col gap-4">
			{auditions.map((audition) => (
				<Card key={audition.id}>
					<CardHeader>
						<CardTitle>{audition.description}</CardTitle>
						<CardDescription>{audition.location}</CardDescription>
					</CardHeader>
					<CardContent>
						<span className="font-medium">Duration:</span>{" "}
						{format(new Date(audition.start_time), "MMM d, yyyy")} —{" "}
						{audition.end_time
							? format(new Date(audition.end_time), "MMM d, yyyy")
							: "TBA"}
					</CardContent>
					{audition.application_url && (
						<CardFooter>
							<a
								rel="noopener noreferrer"
								target="_blank"
								href={audition.application_url}
							>
								<Button size="sm" variant="outline" className="cursor-pointer">
									Apply
								</Button>
							</a>
						</CardFooter>
					)}
				</Card>
			))}
		</div>
	);
}
