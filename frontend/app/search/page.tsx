export default async function Settings({
	searchParams,
}: {
	searchParams: { [key: string]: string | string[] | undefined };
}) {
	const query = searchParams.query as string;

	return <h1>hello? {query}</h1>;
}
