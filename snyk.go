package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"net/http/httputil"
	"strings"

	"github.com/prometheus/common/log"
)

type client struct {
	httpClient *http.Client
	token      string
	baseURL    string
}

func (c *client) getOrganizations() (orgsResponse, error) {
	req, err := http.NewRequest(http.MethodGet, fmt.Sprintf("%s/rest/orgs", c.baseURL), nil)
	if err != nil {
		return orgsResponse{}, err
	}
	response, err := c.do(req)
	if err != nil {
		return orgsResponse{}, err
	}
	var orgs orgsResponse
	err = json.NewDecoder(response.Body).Decode(&orgs)
	if err != nil {
		return orgsResponse{}, err
	}
	return orgs, nil
}

func (c *client) getProjects(organization string) (projectsResponse, error) {
	req, err := http.NewRequest(http.MethodGet, fmt.Sprintf("%s/rest/orgs/%s/projects", c.baseURL, organization), nil)
	if err != nil {
		return projectsResponse{}, err
	}
	response, err := c.do(req)
	if err != nil {
		return projectsResponse{}, err
	}
	var projectsResponseObject projectsResponse
	err = json.NewDecoder(response.Body).Decode(&projectsResponseObject)
	if err != nil {
		return projectsResponse{}, err
	}
	var projects []project
	projects = append(projects, projectsResponseObject.Projects...)
	for !strings.EqualFold(projectsResponseObject.Links.Next, "") {
		req, err := http.NewRequest(http.MethodGet, fmt.Sprintf("%s%s", c.baseURL, projectsResponseObject.Links.Next), nil)
		if err != nil {
			return projectsResponse{}, err
		}
		response, err := c.do(req)
		if err != nil {
			return projectsResponse{}, err
		}
		var projectsResponseObject projectsResponse
		err = json.NewDecoder(response.Body).Decode(&projectsResponseObject)
		if err != nil {
			return projectsResponse{}, err
		}
		projects = append(projects, projectsResponseObject.Projects...)
	}
	return projectsResponse{projects, nil}, nil
}

func (c *client) getIssues(organizationID, projectID string) (issuesResponse, error) {

	req, err := http.NewRequest(http.MethodGet, fmt.Sprintf("%s/rest/orgs/%s/issues", c.baseURL, organizationID), nil)
	if err != nil {
		return issuesResponse{}, err
	}
  // Find any issues that has a relation to this project ID
	query := req.URL.Query()
	query.Set("scan_item.id", projectID)
	query.Set("scan_item.type", "project")
	req.URL.RawQuery = query.Encode()

	response, err := c.do(req)
	if err != nil {
		return issuesResponse{}, err
	}
	var issuesResponseObject issuesResponse
	err = json.NewDecoder(response.Body).Decode(&issuesResponseObject)
	if err != nil {
		return issuesResponse{}, err
	}
	var issues []issue
	issues = append(issues, issuesResponseObject.Issues...)
	for !strings.EqualFold(issuesResponseObject.Links.Next, "") {
		req, err := http.NewRequest(http.MethodGet, fmt.Sprintf("%s%s", c.baseURL, issuesResponseObject.Links.Next), nil)
		if err != nil {
			return issuesResponse{}, err
		}
		response, err := c.do(req)
		if err != nil {
			return issuesResponse{}, err
		}
		var issuesResponseObject issuesResponse
		err = json.NewDecoder(response.Body).Decode(&issuesResponseObject)
		if err != nil {
			return issuesResponse{}, err
		}
		issues = append(issues, issuesResponseObject.Issues...)
	}
	return issuesResponse{issues, nil}, nil
}

func (c *client) do(req *http.Request) (*http.Response, error) {
	req.Header.Add("authorization", fmt.Sprintf("TOKEN %s", c.token))

	query := req.URL.Query()
	query.Set("version", "2024-01-23")
	query.Set("limit", "100")
	req.URL.RawQuery = query.Encode()

	response, err := c.httpClient.Do(req)
	if err != nil {
		return nil, err
	}
	if response.StatusCode != http.StatusOK {
		body, err := ioutil.ReadAll(response.Body)
		if err != nil {
			log.Errorf("read body failed: %v", err)
			body = []byte("failed to read body")
		}
		requestDump, err := httputil.DumpRequestOut(req, true)
		if err != nil {
			log.Debugf("Failed to dump request for logging")
		} else {
			log.Debugf("Failed request dump: %s", requestDump)
		}
		return nil, fmt.Errorf("request not OK: %s: url: %s body: %s", response.Status, req.URL, body)
	}
	return response, nil
}

type orgsResponse struct {
	Orgs  []org `json:"data,omitempty"`
	Links *struct {
		Next string `next:"next,omitempty"`
	} `json:"links,omitempty"`
}

type org struct {
	ID         string `json:"id,omitempty"`
	Type       string `json:"type,omitempty"`
	Attributes *struct {
		GroupID string `json:"group_id,omitempty"`
		Name    string `json:"name,omitempty"`
	} `json:"attributes,omitempty"`
}

type projectsResponse struct {
	Projects []project `json:"data,omitempty"`
	Links    *struct {
		Next string `next:"next,omitempty"`
	} `json:"links,omitempty"`
}

type project struct {
	ID         string `json:"id,omitempty"`
	Type       string `json:"type,omitempty"`
	Attributes *struct {
		Name string `json:"name,omitempty"`
	} `json:"attributes,omitempty"`
}

type issuesResponse struct {
	Issues []issue `json:"data,omitempty"`
	Links  *struct {
		Next string `next:"next,omitempty"`
	} `json:"links,omitempty"`
}

type issue struct {
	ID          string      `json:"id,omitempty"`
	IssueType   string      `json:"issueType"`
	Severity    string      `json:" effective_severity_level,omitempty"`
	Title       string      `json:"title,omitempty"`
	Ignored     bool        `json:"ignored"`
	Coordinates coordinates `json:"coordinates,omitempty"`
}

type coordinates struct {
	Upgradeable bool `json:"is_upgradable"`
	Patchable   bool `json:"is_patchable"`
}

type license struct{}