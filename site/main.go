package main

import (
	"fmt"
	"net/http"
)

func main() {
	mux := http.NewServeMux()
	fs := http.FileServer(http.Dir("static"))

	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, "templates/index.html")
	})

	mux.HandleFunc("/favicon.ico", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "image/x-icon; charset=utf-8")
		http.ServeFile(w, r, "favicon.ico")
	})

	mux.Handle("/static/", http.StripPrefix("/static/", fs))

	err := http.ListenAndServe(":8080", mux)
	if err != nil {
		fmt.Println("Unable to start server")
		return
	}
}
