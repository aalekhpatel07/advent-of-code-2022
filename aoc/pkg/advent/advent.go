package advent

import (
	"aoc/pkg/structs"
	"errors"
	"fmt"
	"io/ioutil"
	"net/http"
	"net/http/cookiejar"
	"net/url"
	"os"
	"strings"
)

func buildClient(urlObj *url.URL) (http.Client, error) {

	sessionId := os.Getenv("AOC_SESSION_ID")
	if sessionId == "" {
		return http.Client{}, errors.New("no AOC_SESSION_ID provided. " +
			"(It can be found by logging in through a browser and getting the session cookie.)",
		)
	}
	jar, err := cookiejar.New(nil)
	if err != nil {
		return http.Client{}, err
	}
	client := http.Client{
		Jar: jar,
	}
	sessionCookie := &http.Cookie{
		Name:  "session",
		Value: os.Getenv("AOC_SESSION_ID"),
	}
	client.Jar.SetCookies(urlObj, []*http.Cookie{sessionCookie})
	return client, nil
}

func GetInputs(day int, year int) ([]structs.Group, error) {

	urlObj, _ := url.Parse(fmt.Sprintf("https://adventofcode.com/%d/day/%d/input", year, day))
	client, err := buildClient(urlObj)
	if err != nil {
		return make([]structs.Group, 0), err
	}
	request, _ := http.NewRequest(http.MethodGet, urlObj.String(), nil)
	resp, err := client.Do(request)

	if err != nil {
		return make([]structs.Group, 0), err
	}
	if resp.StatusCode != 200 {
		text, err := ioutil.ReadAll(resp.Body)
		if err != nil {
			return make([]structs.Group, 0), err
		}
		return make([]structs.Group, 0), errors.New(string(text))
	}
	responseText, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return make([]structs.Group, 0), err
	}
	contentsByGroup := strings.Split(string(responseText), "\n\n")

	results := make([]structs.Group, len(contentsByGroup))

	for idx, content := range contentsByGroup {
		results[idx] = structs.Group{
			Contents: content,
		}
	}
	return results, nil
}

func PostAnswer(day int, year int, part int, answer string) (bool, error) {

	urlObj, _ := url.Parse(fmt.Sprintf("https://adventofcode.com/%d/day/%d/answer", year, day))
	client, err := buildClient(urlObj)
	if err != nil {
		return false, err
	}
	data := url.Values{
		"level":  {fmt.Sprintf("%d", part)},
		"answer": {answer},
	}
	resp, err := client.PostForm(urlObj.String(), data)
	if err != nil {
		return false, err
	}

	if resp.StatusCode != 200 {
		text, err := ioutil.ReadAll(resp.Body)
		if err != nil {
			return false, err
		}
		return false, errors.New(string(text))
	}

	responseText, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return false, err
	}
	text := string(responseText)

	if strings.Contains(text, "That's not the right answer.") {
		return false, errors.New(text)
	}
	return true, nil
}
