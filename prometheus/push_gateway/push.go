package push_gateway

import (
	"fmt"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/push"
	"math/rand"
	"prometheus/config"
	"sync"
	"time"
)

var pusher *push.Pusher

func init() {
	pusher = push.New(config.PushGatewayAddr, config.JobName)
}

func MockPush() {
	wg := sync.WaitGroup{}
	wg.Add(4)

	counter := prometheus.NewCounter(prometheus.CounterOpts{
		Name: "my_counter",
		Help: "it is a test counter",
	})
	counterPusher := pusher.Collector(counter).Grouping("group", "counter-g").Grouping("instance", "cargo_demo")

	gauge := prometheus.NewGauge(prometheus.GaugeOpts{
		Name: "my_gauge",
		Help: "it is a test gauge",
	})
	gaugePusher := pusher.Collector(gauge).Grouping("group", "gauge-g").Grouping("instance", "cargo_demo")

	histogramVec := prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Name:    "my_histogram",
			Help:    "it is a test histogram",
			Buckets: []float64{0, 10, 30, 50, 80, 100, 150, 200, 500, 1000, 1500, 2000},
		},
		[]string{"method", "path"},
	)
	histogramPusher := pusher.Collector(histogramVec).Grouping("group", "histogram-g").Grouping("instance", "cargo_demo")

	summaryVec := prometheus.NewSummaryVec(prometheus.SummaryOpts{
		Name:       "my_summary",
		Help:       "it is a test summary",
		Objectives: map[float64]float64{0.2: 50, 0.5: 100, 0.9: 1000, 0.99: 2100},
	},
		[]string{"method", "path"},
	)
	summaryPusher := pusher.Collector(summaryVec).Grouping("group", "summary-g").Grouping("instance", "cargo_demo")

	go func() {
		ticker := time.NewTicker(time.Second * 10)

		var err error
		for {
			select {
			case <-ticker.C:
				fmt.Println("==> start push ", time.Now())
				err = counterPusher.Push()
				if err != nil {
					fmt.Println("counter push err ", err)
					err = nil
				}

				err = gaugePusher.Push()
				if err != nil {
					fmt.Println("gauge push err ", err)
					err = nil
				}

				err = histogramPusher.Push()
				if err != nil {
					fmt.Println("histogram push err ", err)
					err = nil
				}

				err = summaryPusher.Push()
				if err != nil {
					fmt.Println("summary push err ", err)
					err = nil
				}

			}
		}

	}()

	// counter
	go func() {
		defer wg.Done()

		fmt.Println("==> counter")
		for i := 0; i <= 100000000; i++ {
			counter.Inc()
			time.Sleep(1 * time.Second)
		}

	}()

	// Gauge
	go func() {
		fmt.Println("==> gauge")
		defer wg.Done()

		for i := 0; i <= 100000000; i++ {
			gauge.Add(0.003)
			time.Sleep(1 * time.Second)
			gauge.Sub(0.002)
		}

	}()

	// histogram
	go func() {
		fmt.Println("==> histogram")
		defer wg.Done()

		for i := 0; i <= 100000000; i++ {
			start := time.Now()
			time.Sleep(time.Millisecond * time.Duration(rand.Int63n(2000)))
			duration := time.Since(start).Milliseconds()

			histogramVec.WithLabelValues("POST", "/index").Observe(float64(duration))
			histogramVec.WithLabelValues("GET", "/index").Observe(float64(duration) + 10)

			time.Sleep(1 * time.Second)
		}

	}()

	// summary
	go func() {
		fmt.Println("==> summary")
		defer wg.Done()
		start := time.Now()
		time.Sleep(time.Millisecond * time.Duration(rand.Int63n(1000)))
		duration := time.Since(start).Milliseconds()

		summaryVec.WithLabelValues("POST", "/list").Observe(float64(duration))
		summaryVec.WithLabelValues("GET", "/list").Observe(float64(duration) + 10)

		time.Sleep(1 * time.Second)

	}()

	fmt.Println("==> wait start")
	wg.Wait()
	fmt.Println("==> wait done")

}
