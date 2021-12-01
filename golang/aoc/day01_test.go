package aoc

import "testing"

func Test_countDepthIncreasesWin(t *testing.T) {
	type args struct {
		measurements []int64
		window       int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "sample",
			args: args{
				measurements: []int64{199, 200, 208, 210, 200, 207, 240, 269, 260, 263},
				window:       2,
			},
			want: 7,
		},
		{
			name: "minimal",
			args: args{
				measurements: []int64{199, 200},
				window:       2,
			},
			want: 1,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := countDepthIncreasesWin(tt.args.measurements, tt.args.window); got != tt.want {
				t.Errorf("countDepthIncreasesWin() = %v, want %v", got, tt.want)
			}
		})
	}
}
