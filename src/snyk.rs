use openapi::models::{Issue, ScanItemType};

use crate::AppArguments;

pub async fn get_organizations(arguments: &AppArguments) -> Vec<String> {
  log::debug!("Start finding organizations");
  let mut configuration = openapi::apis::configuration::Configuration::default();
  configuration.api_key = Some(openapi::apis::configuration::ApiKey {
      prefix: Some("Token".to_string()),
      key: arguments.snyk_api_token.clone(),
  });

  let mut all_organizations = Vec::new();
  let mut starting_after = None;

  loop {
		  log::debug!("More organizations to be found, currently: {}", all_organizations.len());
      let organizations = openapi::apis::orgs_api::list_orgs(
          &configuration,
          "2024-04-22",
          starting_after.as_deref(),
          None, Some(100), None, None, None, None, None,
      )
      .await
      .unwrap();

      all_organizations.extend(organizations.data.iter().map(|org| org.id.to_string()));

      let next = organizations.links.next.clone();
      if next.is_none() {
          break;
      }

      starting_after = get_next_starting_after(*next.unwrap());

      if starting_after.is_none() {
          break;
      }
  }
  log::debug!("Done finding organizations, found {}", all_organizations.len());

  return all_organizations;
}

pub async fn get_projects(arguments: &AppArguments, organization_id: String) -> Vec<String> {
  log::debug!("Start finding projects for: {}", organization_id);
  let mut configuration = openapi::apis::configuration::Configuration::default();
  configuration.api_key = Some(openapi::apis::configuration::ApiKey {
      prefix: Some("Token".to_string()),
      key: arguments.snyk_api_token.clone(),
  });

  let mut all_projects = Vec::new();
  let mut starting_after = None;

  loop {
      log::debug!("More projects to be found, currently: {}", all_projects.len());
      let projects = openapi::apis::projects_api::list_org_projects(
          &configuration,
          organization_id.as_str(),
          "2024-04-22",
          None, None, None, None, None, None, None, None, None, None,
          None, None, None, None, None, None, None, None, None, None,
          starting_after.as_deref(), None, Some(100),
      )
      .await
      .unwrap();

      let project_list = projects.data.unwrap().clone();
      log::debug!("Found {} projects", project_list.len());

      all_projects.extend(project_list.iter().map(|project| project.id.to_string()));

      let next = projects.links.next.clone();
      if next.is_none() {
          break;
      }

      starting_after = get_next_starting_after(*next.unwrap());

      if starting_after.is_none() {
          break;
      }
  }
  log::debug!("Done finding projects, found {}", all_projects.len());

  return all_projects;
}

pub async fn get_issues(
  arguments: &AppArguments,
  organization_id: String,
  project_id: String,
) -> Vec<Issue> {
  log::debug!("Start finding issues for: {}", project_id);
  let mut configuration = openapi::apis::configuration::Configuration::default();
  configuration.api_key = Some(openapi::apis::configuration::ApiKey {
      prefix: Some("Token".to_string()),
      key: arguments.snyk_api_token.clone(),
  });

  let mut all_issues = Vec::new();
  let mut starting_after = None;

  loop {
    log::debug!("More issues to be found, currently: {}", all_issues.len());
      let issues = openapi::apis::issues_api::list_org_issues(
          &configuration,
          "2024-04-22",
          organization_id.as_str(),
          starting_after.as_deref(),
          None,
          Some(100),
          Some(project_id.as_str()),
          Some(ScanItemType::Project),
          None, None, None, None, None, None, None, None,
      )
      .await
      .unwrap();

      all_issues.extend(issues.data.iter().map(|issue| issue.clone()));

      let next = issues.links.unwrap().next.clone();
      if next.is_none() {
          break;
      }

      starting_after = get_next_starting_after(*next.unwrap());

      if starting_after.is_none() {
          break;
      }
  }
  log::debug!("Done finding issues, found {}", all_issues.len());

  return all_issues;
}

fn get_next_starting_after(next_link: openapi::models::LinkProperty) -> Option<String> {
  if let openapi::models::LinkProperty::String(next_url) = next_link {
      let queries = querystring::querify(next_url.as_str());
      log::debug!("Found next url: {:?}", next_url);
      if let Some(entry) = queries.iter().find(|&&(key, _)| key == "starting_after") {
          log::info!("Next starting_after: {:?}", entry);
          return Some(urlencoding::decode(entry.1).unwrap().to_string());
      } else {
          return None;
      }
  }
  return None;
}
