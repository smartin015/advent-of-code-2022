#!/usr/bin/perl
use warnings;
use strict;
use List::Util qw/sum/;
use Array::Utils qw(:all);

my @data = <STDIN>;
my ($split_idx) = grep { $data[$_] eq "\n" } (0 .. @data-1);

print("Splitting data at $split_idx\n");

sub load_stacks {
  my @input = ();
  for (@data[0..$split_idx-1]) {
    chomp $_;
    print("LS $_\n");
    push(@input, $_);
  }
  # Reverse collected input to create containers as arrays of strings
  my $slen = split(' ', $input[-1]);
  print("Len $slen\n");
  my @stacks = ("") x $slen;

  my @rest = reverse(@input[0..$#input-1]);
  for my $ri (@rest) {
    # my @boxes = ($ri =~ /[(\w+)]/g);
    for my $i (0 .. $slen-1) {
      my $box = substr($ri, 1 + 4*$i, 1);
      # print("Box at pos $i: $box\n");
      if ($box ne " ") {
        $stacks[$i] = $stacks[$i] . $box;
      }
    }
  }
  return @stacks;
}

sub load_instructions {
  my @inst = ();
  for (@data[$split_idx+1..$#data]) {
    chomp $_;
    print("LI $_\n");
    if ($_ eq "") {
      last;
    }
    my @all_nums = ($_ =~ /(\d+)/g);
    push(@inst, \@all_nums);
  }
  return @inst;
}

my @stacks = load_stacks();
sub print_stacks {
  for my $s (@stacks) {
    print("Stack: $s\n");
  }
}
print_stacks();

my @inst = load_instructions();
for (@inst) {
  print("Instruction: @$_\n");
}

sub mv_crate_individual {
  my $num = int($_[0]);
  my $from = int($_[1]) - 1;
  my $to = int($_[2]) - 1;
  for my $i (0..$num-1) {
    print("I is $i\n");
    my $b = chop($stacks[$from]);
    print("b is $b\n");
    $stacks[$to] = $stacks[$to] . $b;
  }
}

sub mv_crate_subset {
  my $num = int($_[0]);
  my $from = int($_[1]) - 1;
  my $to = int($_[2]) - 1;
  my $b = substr($stacks[$from], -$num, $num);
  print("b is $b\n");
  $stacks[$from] = substr($stacks[$from], 0, -$num);
  $stacks[$to] = $stacks[$to] . $b;
}

for (@inst) {
  my @unref = @$_;
  print("Inst: @unref\n");
  mv_crate_subset($unref[0], $unref[1], $unref[2]);
}
print_stacks();

print("Result:\n");
for (@stacks) {
  print(substr($_, -1, 1));
}
print("\n");
