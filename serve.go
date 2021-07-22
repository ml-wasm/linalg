package main

import (
	"fmt"
	"net/http"
)

func addHeaders(handler http.Handler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Cross-Origin-Opener-Policy", "same-origin")
		w.Header().Set("Cross-Origin-Embedder-Policy", "require-corp")

		handler.ServeHTTP(w, r)
	}
}

func main() {
	fsHandler := http.FileServer(http.Dir("./"))
	http.Handle("/", addHeaders(fsHandler))

	fmt.Println("🚀 Listening on http://localhost:5000/")
	http.ListenAndServe(":5000", nil)
}
