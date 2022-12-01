#!/usr/bin/perl
use warnings;
use strict;
use List::Util qw/sum/;

my $acc = 0;
my @cals = ();
for (<STDIN>) {
  chomp $_;
	if (length($_) == 0) {
		push(@cals, $acc);
		$acc = 0;
	} else {
    $acc += int($_);
  }
}
if ($acc > 0) {
	push(@cals, $acc);
}

@cals = sort { $a <=> $b } @cals;
print("Max is: $cals[-1]\n");

my @top3 = @cals[-3..-1];
print("Top 3: " . join(', ', @top3) . "\n");
print("Top 3 total: " . sum(@top3) . "\n");
