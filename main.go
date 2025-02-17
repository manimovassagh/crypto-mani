package main

import (
	"context"
	"crypto/sha256"
	"fmt"
	"runtime"
	"strconv"
	"strings"
	"sync"
)

const difficulty = 6 // Number of leading zeros required

// Miner struct to encapsulate mining logic
type Miner struct {
	data       string
	numWorkers int
	ctx        context.Context
	cancel     context.CancelFunc
	found      chan string
	wg         sync.WaitGroup
}

// NewMiner initializes a new Miner
func NewMiner(data string) *Miner {
	ctx, cancel := context.WithCancel(context.Background())
	return &Miner{
		data:       data,
		numWorkers: runtime.NumCPU(), // Automatically detect number of CPU cores
		ctx:        ctx,
		cancel:     cancel,
		found:      make(chan string, 1),
	}
}

// mineBlock is the worker function
func (m *Miner) mineBlock(id, startNonce, step int) {
	defer m.wg.Done()
	nonce := startNonce

	for {
		select {
		case <-m.ctx.Done():
			return // Stop when mining is done
		default:
			hash := sha256.Sum256([]byte(m.data + strconv.Itoa(nonce)))
			hashStr := fmt.Sprintf("%x", hash)

			if strings.HasPrefix(hashStr, strings.Repeat("0", difficulty)) {
				m.found <- fmt.Sprintf("Worker %d found a valid hash: %s (Nonce: %d)", id, hashStr, nonce)
				m.cancel() // Stop all goroutines
				return
			}
			nonce += step
		}
	}
}

// StartMining runs the mining process
func (m *Miner) StartMining() string {
	fmt.Printf("Starting mining with %d workers...\n", m.numWorkers)

	for i := 0; i < m.numWorkers; i++ {
		m.wg.Add(1)
		go m.mineBlock(i, i, m.numWorkers)
	}

	result := <-m.found // Wait for first valid hash
	m.wg.Wait()         // Ensure all goroutines exit properly
	return result
}

func main() {
	miner := NewMiner("Mani's Block is working nice")
	result := miner.StartMining()
	fmt.Println(result)
}