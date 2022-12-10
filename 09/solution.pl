#!/usr/bin/perl
use warnings;
use strict;
use Data::Dumper;
use List::Util qw(max);


my %dirs = (
  'R' => [1,0],
  'L' => [-1,0],
  'U' => [0,1],
  'D' => [0,-1],
);
  
my @rope = (
  [0,0], # H
  [0,0], # 1
  [0,0], # ...
  [0,0], 
  [0,0], 
  [0,0], 
  [0,0], 
  [0,0], 
  [0,0], 
  [0,0], # 9
  [0,0], # s
);
my $ropelen = 10;

my %seen = (
  "0,0" => 1
);

sub display {
  my @mm = ();
  for my $r (0..25) {
    my @m;
    for my $c (0..25) {
      push(@m, '.');
    }
    push(@mm,\@m);
  }

  my @order = reverse(0..$ropelen);
  for (@order) {
    my $c = "H";
    if ($_ == 10) {
      $c = 's';
    } elsif ($_ != 0) {
      $c = "$_";
    }
    my $x = $rope[$_][0] + 11;
    my $y = $rope[$_][1] + 10;
    my $l = scalar(@mm)-1;
    @mm[$l-$y]->[$x] = $c;
  }
  for my $r (@mm) {
    print("@$r\n");
  }
}


# print("Start pos " . Dumper(@rope) . "\n");
for (<STDIN>) {
  print(">: $_");
  my @split = split(' ', $_);
  my $rcmd = $dirs{$split[0]};
  my @cmd = @$rcmd;
  my $mult = int($split[1]);
  for my $rep (0..$mult-1) {
    # Move the head
    $rope[0][0] = $rope[0][0]+$cmd[0];
    $rope[0][1] = $rope[0][1]+$cmd[1];
    
    # Loop through each part of the rope and pull it towards the earlier part, if needed
    for my $i (1..$ropelen) {
      my $dx = $rope[$i-1][0] - $rope[$i][0];
      my $dy = $rope[$i-1][1] - $rope[$i][1];
      my $mx = max(abs($dx), 1);
      my $my = max(abs($dy), 1);

      # print("i $i; dx $dx; dy $dy\n");
      if ($mx + $my > 2) {
        # Move diagonally
        # print("Move diag " . ($dx/$mx) . "," . ($dy/$my) . "\n");
        $rope[$i][0] += ($dx/$mx);
        $rope[$i][1] += ($dy/$my);
      } elsif ($mx > 1 || $my > 1) {
        # Move orthogonally
        $rope[$i][0] = $rope[$i-1][0] - ($dx/$mx);
        $rope[$i][1] = $rope[$i-1][1] - ($dy/$my);
      }
    }
    # -2 because we want to track '9', not 's'
    $seen{"$rope[-2][0],$rope[-2][1]"} = 1;
  }

  # print("mv ($cmd[0], $cmd[1]) -> now " . Dumper(@rope) . "\n");
  # print("seen " . keys(%seen) . "\n");
}
# display();
print("seen " . keys(%seen) . "\n");
