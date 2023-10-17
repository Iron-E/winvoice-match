use winvoice_schema::Job;

use super::{
	DateTime,
	Duration,
	Id,
	Match,
	MatchDepartment,
	MatchInvoice,
	MatchJob,
	MatchOption,
	MatchOrganization,
	MatchSet,
	MatchStr,
	Serde,
	Utc,
};

impl From<Duration> for MatchJob
{
	fn from(duration: Duration) -> Self
	{
		Match::from(Serde::from(duration)).into()
	}
}

impl From<Id> for MatchJob
{
	fn from(id: Id) -> Self
	{
		Match::from(id).into()
	}
}

impl From<Job> for MatchJob
{
	fn from(job: Job) -> Self
	{
		Self {
			client: job.client.into(),
			id: job.id.into(),
			notes: job.notes.into(),
			invoice: job.invoice.into(),
			date_open: job.date_open.into(),
			increment: Serde::from(job.increment).into(),
			date_close: job.date_close.map(Into::into).into(),
			objectives: job.objectives.into(),
			departments: job.departments.into_iter().map(Into::into).collect(),
		}
	}
}

impl From<DateTime<Utc>> for MatchJob
{
	fn from(date: DateTime<Utc>) -> Self
	{
		Match::from(date).into()
	}
}

impl From<Match<Id>> for MatchJob
{
	fn from(id: Match<Id>) -> Self
	{
		Self { id, ..Default::default() }
	}
}

impl From<Match<DateTime<Utc>>> for MatchJob
{
	fn from(date_open: Match<DateTime<Utc>>) -> Self
	{
		Self { date_open, ..Default::default() }
	}
}

impl From<Match<Serde<Duration>>> for MatchJob
{
	fn from(increment: Match<Serde<Duration>>) -> Self
	{
		Self { increment, ..Default::default() }
	}
}

impl From<MatchDepartment> for MatchJob
{
	fn from(departments: MatchDepartment) -> Self
	{
		MatchSet::from(departments).into()
	}
}

impl From<MatchInvoice> for MatchJob
{
	fn from(invoice: MatchInvoice) -> Self
	{
		Self { invoice, ..Default::default() }
	}
}

impl From<MatchOrganization> for MatchJob
{
	fn from(client: MatchOrganization) -> Self
	{
		Self { client, ..Default::default() }
	}
}

impl From<MatchOption<Match<DateTime<Utc>>>> for MatchJob
{
	fn from(date_close: MatchOption<Match<DateTime<Utc>>>) -> Self
	{
		Self { date_close, ..Default::default() }
	}
}

impl From<MatchSet<MatchDepartment>> for MatchJob
{
	fn from(departments: MatchSet<MatchDepartment>) -> Self
	{
		Self { departments, ..Default::default() }
	}
}

impl From<MatchStr<String>> for MatchJob
{
	fn from(objectives: MatchStr<String>) -> Self
	{
		Self { objectives, ..Default::default() }
	}
}

impl From<Option<Match<DateTime<Utc>>>> for MatchJob
{
	fn from(date_close: Option<Match<DateTime<Utc>>>) -> Self
	{
		MatchOption::from(date_close).into()
	}
}
