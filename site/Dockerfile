FROM golang:1.22

WORKDIR /sites/public-page

COPY go.mod .

RUN go mod download

COPY . .

RUN go build -o main .

CMD ["./main"]
