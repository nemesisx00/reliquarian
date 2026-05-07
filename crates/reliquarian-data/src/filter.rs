use freya::radio::RadioChannel;
use serde::{Deserialize, Serialize};
use crate::enums::GamePlatforms;

pub trait Filterable<T>
{
	fn filter(&self, text: impl Into<String>, filter: impl Into<FilterCriteria>) -> Vec<T>;
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FilterCriteria
{
	/// Should the text search be case sensitive
	#[serde(default)]
	pub caseSensitive: bool,
	
	/// Display only locked or unlocked
	#[serde(default)]
	pub locked: bool,
	
	/// Should the text be searched for in the name only
	#[serde(default)]
	pub nameOnly: bool,
	
	/// Show all items when there is a boolean test that could hide some (i.e. games with no achievements metadata)
	#[serde(default)]
	pub showAll: bool,
}

impl RadioChannel<FilterCriteria> for GamePlatforms {}
