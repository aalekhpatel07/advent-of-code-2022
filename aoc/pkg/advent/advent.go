package advent

import (
	"errors"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"net/http/cookiejar"
	"net/url"
	"os"
	"strings"

	"aoc/pkg/structs"
)

func buildClient(urlObj *url.URL) (*http.Client, error) {

	sessionId := os.Getenv("AOC_SESSION_ID")
	if sessionId == "" {
		return nil, errors.New("no AOC_SESSION_ID provided. " +
			"(It can be found by logging in through a browser and getting the session cookie.)",
		)
	}
	jar, err := cookiejar.New(nil)
	if err != nil {
		return nil, err
	}
	client := http.Client{
		Jar: jar,
	}
	sessionCookie := &http.Cookie{
		Name:  "session",
		Value: os.Getenv("AOC_SESSION_ID"),
	}
	client.Jar.SetCookies(urlObj, []*http.Cookie{sessionCookie})
	return &client, nil
}

func GetInputs(day int, year int) ([]structs.Group, error) {

	urlObj, _ := url.Parse(fmt.Sprintf("https://adventofcode.com/%d/day/%d/input", year, day))
	client, err := buildClient(urlObj)
	if err != nil {
		return nil, err
	}
	request, _ := http.NewRequest(http.MethodGet, urlObj.String(), nil)
	resp, err := client.Do(request)

	if err != nil {
		return nil, err
	}
	// Hmm `defer resp.Body.Close()` complains about an unhandled error.
	// So we drop the error in a closure?
	defer func(Body io.ReadCloser) {
		_ = Body.Close()
	}(resp.Body)

	if resp.StatusCode != 200 {
		text, err := ioutil.ReadAll(resp.Body)
		if err != nil {
			return make([]structs.Group, 0), err
		}
		return nil, errors.New(string(text))
	}
	responseText, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
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

	urlObj, err := url.Parse(fmt.Sprintf("https://adventofcode.com/%d/day/%d/answer", year, day))
	if err != nil {
		return false, err
	}
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

	// Hmm `defer resp.Body.Close()` complains about an unhandled error.
	// So we drop the error in a closure?
	defer func(Body io.ReadCloser) {
		_ = Body.Close()
	}(resp.Body)

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
