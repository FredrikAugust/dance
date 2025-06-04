import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

export default function Search() {
	return (
		<div className="flex gap-2">
			<form action="/search" method="GET" className="contents">
				<Input
					placeholder="Search for a dance audition"
					name="query"
					autoComplete="off"
				/>
				<Button>Search</Button>
			</form>
		</div>
	);
}
