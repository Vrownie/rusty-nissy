#include "commands.h"

/* Arg parsing functions *****************************************************/

CommandArgs *           solve_parse_args(int c, char **v);
CommandArgs *           help_parse_args(int c, char **v);
CommandArgs *           print_parse_args(int c, char **v);
CommandArgs *           parse_no_arg(int c, char **v);

/* Exec functions ************************************************************/

static void             solve_exec(CommandArgs *args);
static void             steps_exec(CommandArgs *args);
static void             commands_exec(CommandArgs *args);
static void             print_exec(CommandArgs *args);
static void             help_exec(CommandArgs *args);
static void             quit_exec(CommandArgs *args);
static void             version_exec(CommandArgs *args);

/* Local functions ***********************************************************/

static bool             read_step(CommandArgs *args, char *str);
static bool             read_scramble(int c, char **v, CommandArgs *args);

/* Commands ******************************************************************/

Command
solve_cmd = {
	.name        = "solve",
	.usage       = "solve STEP [OPTIONS] SCRAMBLE",
	.description = "Solve a step; see command steps for a list of steps",
	.parse_args  = solve_parse_args,
	.exec        = solve_exec
};

Command
steps_cmd = {
	.name        = "steps",
	.usage       = "steps",
	.description = "List available steps",
	.parse_args  = parse_no_arg,
	.exec        = steps_exec
};

Command
commands_cmd = {
	.name        = "commands",
	.usage       = "commands",
	.description = "List available commands",
	.parse_args  = parse_no_arg,
	.exec        = commands_exec
};

Command
print_cmd = {
	.name        = "print",
	.usage       = "print SCRAMBLE",
	.description = "Print written description of the cube",
	.parse_args  = print_parse_args,
	.exec        = print_exec,
};

Command
help_cmd = {
	.name        = "help",
	.usage       = "help [COMMAND]",
	.description = "Display nissy manual page or help on specific command",
	.parse_args  = help_parse_args,
	.exec        = help_exec,
};

Command
quit_cmd = {
	.name        = "quit",
	.usage       = "quit",
	.description = "Quit nissy",
	.parse_args  = parse_no_arg,
	.exec        = quit_exec,
};

Command
version_cmd = {
	.name        = "version",
	.usage       = "version",
	.description = "print nissy version",
	.parse_args  = parse_no_arg,
	.exec        = version_exec,
};

Command *commands[NCOMMANDS] = {
	&commands_cmd,
	&help_cmd,
	&print_cmd,
	&quit_cmd,
	&solve_cmd,
	&steps_cmd,
	&version_cmd,
};

/* Arg parsing functions implementation **************************************/

CommandArgs *
solve_parse_args(int c, char **v)
{
	int i;
	long val;

	CommandArgs *a = malloc(sizeof(CommandArgs));

	a->success  = false;
	a->opts     = malloc(sizeof(SolveOptions));
	a->step     = steps[0];
	a->command  = NULL;
	a->scramble = NULL;

	a->opts->min_moves     = 0;
	a->opts->max_moves     = 20;
	a->opts->max_solutions = 1;
	a->opts->nthreads      = 1;
	a->opts->optimal_only  = false;
	a->opts->can_niss      = false;
	a->opts->verbose       = false;
	a->opts->all           = false;
	a->opts->print_number  = true;

	for (i = 0; i < c; i++) {
		if (!strcmp(v[i], "-m")) {
			val = strtol(v[++i], NULL, 10);
			if (val < 0 || val > 100) {
				fprintf(stderr,
					"Invalid min number of moves"
					"(0 <= m <= 100).\n");
				return a;
			}
			a->opts->min_moves = val;
		} else if (!strcmp(v[i], "-M")) {
			val = strtol(v[++i], NULL, 10);
			if (val < 0 || val > 100) {
				fprintf(stderr,
					"Invalid max number of moves"
					"(0 <= M <= 100).\n");
				return a;
			}
			a->opts->max_moves = val;
		} else if (!strcmp(v[i], "-t")) {
			val = strtol(v[++i], NULL, 10);
			if (val < 1 || val > 64) {
				fprintf(stderr,
					"Invalid number of threads."
					"1 <= t <= 64\n");
				return a;
			}
			a->opts->nthreads = val;
		} else if (!strcmp(v[i], "-s")) {
			val = strtol(v[++i], NULL, 10);
			if (val < 1 || val > 1000000) {
				fprintf(stderr,
					"Invalid number of solutions.\n");
				return a;
			}
			a->opts->max_solutions = val;
		} else if (!strcmp(v[i], "-o")) {
			a->opts->optimal_only = true;
		} else if (!strcmp(v[i], "-n")) {
			a->opts->can_niss = true;
		} else if (!strcmp(v[i], "-v")) {
			a->opts->verbose = true;
		} else if (!strcmp(v[i], "-a")) {
			a->opts->all = true;
		} else if (!strcmp(v[i], "-p")) {
			a->opts->print_number = false;
		} else if (!read_step(a, v[i])) {
			break;
		}
	}

	a->success = read_scramble(c-i, &v[i], a);
	return a;
}

CommandArgs *
help_parse_args(int c, char **v)
{
	int i;
	CommandArgs *a = malloc(sizeof(CommandArgs));

	a->scramble = NULL;
	a->opts     = NULL;
	a->step     = NULL;
	a->command  = NULL;

	if (c == 1) {
		for (i = 0; i < NCOMMANDS; i++)
			if (commands[i] != NULL &&
			    !strcmp(v[0], commands[i]->name))
				a->command = commands[i];
		if (a->command == NULL)
			fprintf(stderr, "%s: command not found\n", v[0]);
	}

	a->success = c == 0 || (c == 1 && a->command != NULL);
	return a;
}

CommandArgs *
parse_no_arg(int c, char **v)
{
	CommandArgs *a = malloc(sizeof(CommandArgs));

	a->success  = true;

	return a;
}

CommandArgs *
print_parse_args(int c, char **v)
{
	CommandArgs *a = malloc(sizeof(CommandArgs));

	a->success = read_scramble(c, v, a);
	return a;
}

/* Exec functions implementation *********************************************/

static void
solve_exec(CommandArgs *args)
{
	Cube c;
	AlgList *sols;

	init_symcoord();

	c = apply_alg(args->scramble, (Cube){0});
	sols = solve(c, args->step, args->opts);

	print_alglist(sols, args->opts->print_number);
	free_alglist(sols);
}

static void
steps_exec(CommandArgs *args)
{
	int i;

	for (i = 0; i < NSTEPS && steps[i] != NULL; i++)
		printf("%-15s %s\n", steps[i]->shortname, steps[i]->name);
}

static void
commands_exec(CommandArgs *args)
{
	int i;

	for (i = 0; i < NCOMMANDS && commands[i] != NULL; i++)
		printf("%s\n", commands[i]->usage);

}

static void
print_exec(CommandArgs *args)
{
	init_moves();
	print_cube(apply_alg(args->scramble, (Cube){0}));
}

static void
help_exec(CommandArgs *args)
{
	if (args->command == NULL) {
		printf(
		       "Use the nissy command \"help COMMAND\" for a short "
		       "description of a specific command.\n"
		       "Use the nissy command \"commands\" for a list of "
		       "available commands.\n"
		       "See the manual page for more details. The manual"
		       " page is available with \"man nissy\" on a UNIX"
		       " system (such a Linux or MacOS) or in pdf and html"
		       " format in the docs folder.\n"
		       "Nissy is available for free at "
		       "https://github.com/sebastianotronto/nissy"
		      );
	} else {
		printf("Command %s: %s\nusage: %s\n", args->command->name,
		       args->command->description, args->command->usage);
	}
}

static void
quit_exec(CommandArgs *args)
{
	exit(0);
}

static void
version_exec(CommandArgs *args)
{
	printf(VERSION"\n");
}

/* Local functions implementation ********************************************/

static bool
read_step(CommandArgs *args, char *str)
{
	int i;

	for (i = 0; i < NSTEPS; i++) {
		if (steps[i] != NULL && !strcmp(steps[i]->shortname, str)) {
			args->step = steps[i];
			return true;
		}
	}

	return false;
}

static bool
read_scramble(int c, char **v, CommandArgs *args)
{
	int i, k, n;
	unsigned int j;
	char *algstr;

	if (new_alg(v[0])->len == 0) {
		fprintf(stderr, "%s: moves or option unrecognized\n", v[0]);
		return false;
	}

	n = 0;
	for(i = 0; i < c; i++)
		n += strlen(v[i]);

	algstr = malloc((n + 1) * sizeof(char));
	k = 0;
	for (i = 0; i < c; i++)
		for (j = 0; j < strlen(v[i]); j++)
			algstr[k++] = v[i][j];
	algstr[k] = 0;

	args->scramble = new_alg(algstr);
	free(algstr);

	if (args->scramble->len == 0)
		fprintf(stderr, "Error reading scramble\n");

	return args->scramble->len > 0;
}

/* Public functions implementation *******************************************/

void
free_args(CommandArgs *args)
{
	if (args == NULL)
		return;

	if (args->scramble != NULL)
		free_alg(args->scramble);
	if (args->opts != NULL)
		free(args->opts);

	/* step and command must not be freed, they are static! */

	free(args);
}
