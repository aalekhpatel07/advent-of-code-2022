package input

import (
	"aoc/pkg/structs"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"net/http/cookiejar"
	"net/url"
	"os"
	"strings"
)

func GetInputs(day int, year int) []structs.Group {

	sessionId := os.Getenv("AOC_SESSION_ID")
	if sessionId == "" {
		log.Fatalf(
			"No AOC_SESSION_ID provided. (It can be found by logging in through a browser and getting the session" +
				" cookie)",
		)
	}

	jar, err := cookiejar.New(nil)
	if err != nil {
		log.Fatalf("%s", err.Error())
	}
	client := http.Client{
		Jar: jar,
	}
	sessionCookie := &http.Cookie{
		Name:  "session",
		Value: os.Getenv("AOC_SESSION_ID"),
	}
	urlObj, _ := url.Parse(fmt.Sprintf("https://adventofcode.com/%d/day/%d/input", year, day))
	client.Jar.SetCookies(urlObj, []*http.Cookie{sessionCookie})
	request, _ := http.NewRequest(http.MethodGet, urlObj.String(), nil)
	resp, err := client.Do(request)

	if err != nil {
		log.Fatalf("Error making http request to AOC: %s\n", err)
	}
	if resp.StatusCode != 200 {
		text, err := ioutil.ReadAll(resp.Body)
		if err != nil {
			log.Fatalf("Could not read response body: %s", err.Error())
		}
		log.Fatalf("AOC sent a bad response: %s\n", string(text))
	}
	responseText, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		log.Fatalf("Could not read response body: %s", err.Error())
	}
	contentsByGroup := strings.Split(string(responseText), "\n\n")

	results := make([]structs.Group, len(contentsByGroup))

	for idx, content := range contentsByGroup {
		results[idx] = structs.Group{
			Contents: content,
		}
	}

	return results
}
