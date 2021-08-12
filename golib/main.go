package main

import (
	"C"
	"crypto/x509"
	"fmt"
	"time"
)

//export PrintCerts
func PrintCerts() bool {
	start := time.Now()
	certs, err := x509.SystemCertPool()
	end := time.Now()
	if err != nil {
		panic(err)
	}
	fmt.Printf("found %d certs in %v\n", len(certs.Subjects()), end.Sub(start))

	return true
}

func main() {}
