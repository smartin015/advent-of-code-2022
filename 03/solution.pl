#!/usr/bin/perl
use warnings;
use strict;
use List::Util qw/sum/;
use Array::Utils qw(:all);

sub item_priority {
  if (ord($_[0]) >= ord('a')) {
    return ord($_[0]) - ord('a') + 1;
  } else {
    return ord($_[0]) - ord('A') + 27;
  }
}

sub part1 {
	my $acc = 0;
	for (<STDIN>) {
		chomp $_;
		print("\n\nLine: " . $_);
		if ($_ eq "") {
			continue;
		}
		my @a = split('', $_);
		my @a1 = splice @a, 0, @a/2;
		print("Split to: @a --- @a1\n");
		#my $com = intersect(@a1, @a)[0];
		# print("Common char: $com\n");
		#my $p = item_priority($com);
		#print("Prio: $p\n");
		#$acc += $p;
	}
	print("Total is: $acc\n");
}

my @input = ();
for (<STDIN>) {
	chomp $_;
	push(@input, $_);
}

my $acc = 0;
for (my $i = 0; $i < @input; $i += 3) {
	my @group = @input[$i..$i+2];
	print("Group $i:\n@group\n");
	my @a1 = split('', @group[0]);
	my @a2 = split('', @group[1]);
	my @a3 = split('', @group[2]);
	my @a12 = intersect(@a1, @a2);
	my @a123 = intersect(@a3, @a12);
	my $com = @a123[0];
	print("$com\n");
	my $p = item_priority($com);
	print("Prio: $p\n");
	$acc += $p;
}
print("Total is: $acc\n");
