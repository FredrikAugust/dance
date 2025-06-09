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
import { format, formatDistanceToNow } from "date-fns";
import { Suspense } from "react";
import Filters from "./Filters";

type Opportunity = {
	id: string;
	company_id: string;
	title: string;
	description: string;
	location: string;
	start_time: string;
	end_time: string | null;
	image_urls: string[];
	application_url: string | null;
	application_deadline: string | null;
};

async function getAuditions(): Promise<Opportunity[]> {
	const response = await fetch(`${API_URL}/opportunities`);
	const data = await response.json();
	return data;
}

export default async function Auditions() {
	const auditionsPromise = getAuditions();
	return (
		<div className="flex flex-col gap-4">
			<Filters />
			<Suspense fallback={<div>Loading...</div>}>
				<AuditionsList auditionsPromise={auditionsPromise} />
			</Suspense>
		</div>
	);
}

async function AuditionsList({
	auditionsPromise,
}: {
	auditionsPromise: Promise<Opportunity[]>;
}) {
	const auditions = await auditionsPromise;
	return (
		<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
			{auditions.map((audition) => (
				<Card key={audition.id} className="flex flex-col">
					<CardHeader>
						<CardTitle className="flex flex-col gap-1">
							<span className="text-xs text-muted-foreground">
								{audition.company_id}
							</span>
							<span>{audition.title}</span>
						</CardTitle>
						<CardDescription className="flex flex-col">
							<span>{audition.location}</span>
							<span>
								{format(new Date(audition.start_time), "MMM d, yyyy")} —{" "}
								{audition.end_time
									? format(new Date(audition.end_time), "MMM d, yyyy")
									: "TBA"}
							</span>
						</CardDescription>
					</CardHeader>
					<CardContent>
						<p>{audition.description}</p>
					</CardContent>
					{audition.application_url && (
						<CardFooter className="mt-auto flex gap-2 items-center justify-between">
							<a
								rel="noopener noreferrer"
								target="_blank"
								href={audition.application_url}
							>
								<Button size="sm" variant="outline" className="cursor-pointer">
									Apply
								</Button>
							</a>

							{audition.application_deadline && (
								<span className="text-sm text-muted-foreground">
									deadline{" "}
									{formatDistanceToNow(
										new Date(audition.application_deadline),
										{
											addSuffix: true,
										},
									)}
								</span>
							)}
						</CardFooter>
					)}
				</Card>
			))}
		</div>
	);
}
