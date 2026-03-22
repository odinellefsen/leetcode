function generateSchedule(teams: string[]): string[][][] {
	if (teams.length % 2 !== 0) {
		teams.push("FakeTeam");
	}

	const numTeams = teams.length;
	const numRounds = numTeams - 1;
	const halfSize = numTeams / 2;

	const schedule: string[][][] = [];
	const teamsCopy = teams.slice(); // Clone the array to avoid modifying the original

	for (let round = 0; round < numRounds; round++) {
		const roundMatches: string[][] = [];

		for (let i = 0; i < halfSize; i++) {
			const team1 = teamsCopy[i];
			const team2 = teamsCopy[numTeams - 1 - i];
			if (team1 !== "FakeTeam" && team2 !== "FakeTeam") {
				roundMatches.push([team1, team2]);
			} else if (team1 === "FakeTeam") {
				roundMatches.push([team2, "FakeTeam"]);
			} else if (team2 === "FakeTeam") {
				roundMatches.push([team1, "FakeTeam"]);
			}
		}

		schedule.push(roundMatches);
		// Rotate the teams for the next round, except the first one
		const lastTeam = teamsCopy.pop();
		if (lastTeam) {
			teamsCopy.splice(1, 0, lastTeam);
		}
	}

	return schedule;
}

// Example usage:

// List of teams
const teams = ["AA", "BB", "CC", "DD", "EE", "FF", "GG", "HH", "II", "JJ"];

// Generate the schedule
const schedule = generateSchedule(teams);

// Display the schedule
for (const [index, round] of schedule.entries()) {
	console.log(`Cross ${index + 1}:`);
	for (const match of round) {
		if (match.includes("FakeTeam")) {
			// Handle bye weeks
			console.log(
				`  ${match.find((team) => team !== "FakeTeam")} has no matchup this Cross.`,
			);
		} else {
			console.log(`  ${match[0]} vs ${match[1]}`);
		}
	}
	console.log("");
}
